extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::extra::DelimSpan;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::meta::parser;
use syn::punctuated::Punctuated;
use syn::Fields::{Named, Unit, Unnamed};
use syn::{
    parse_macro_input, parse_str, Field, FieldMutability, ItemStruct, LitStr, Token, Type,
    TypeTuple, Visibility,
};

use copper::config::CopperConfig;
use format::{highlight_rust_code, rustfmt_generated_code};

mod format;
mod utils;

// Parses the CopperRuntime attribute like #[copper_runtime(config = "path")]
#[proc_macro_attribute]
pub fn copper_runtime(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);

    let mut config_file: Option<LitStr> = None;
    let attribute_config_parser = parser(|meta| {
        if meta.path.is_ident("config") {
            config_file = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported tea property"))
        }
    });

    parse_macro_input!(args with attribute_config_parser);
    let config_file = config_file
        .expect("Expected config file attribute like #[CopperRuntime(config = \"path\")]")
        .value();
    let mut config_full_path = utils::caller_crate_root();
    config_full_path.push(&config_file);
    let config_content = std::fs::read_to_string(&config_full_path)
        .unwrap_or_else(|_| panic!("Failed to read configuration file: {:?}", &config_full_path));

    let copper_config = CopperConfig::deserialize(&config_content);
    let all_tasks_configs = copper_config.get_all_nodes();

    // Collect all the type names used by our configs.
    let all_configs_type_names: Vec<String> = all_tasks_configs
        .iter()
        .map(|node_config| node_config.get_type_name().to_string())
        .collect();

    // Transform them as Rust types
    let all_configs_types: Vec<Type> = all_configs_type_names
        .iter()
        .map(|name| {
            println!("Found type: {}", name);
            parse_str(name).unwrap()
        })
        .collect();

    // Construct the path for `copper::runtime::CopperRuntime<CT>` with CT being all the tuple of oll the tasks instances types in order by index
    let copper_runtime_path = syn::Path {
        leading_colon: Some(Token![::](Span::call_site())),
        segments: vec![
            syn::PathSegment {
                ident: Ident::new("copper", Span::call_site()),
                arguments: syn::PathArguments::None,
            },
            syn::PathSegment {
                ident: Ident::new("runtime", Span::call_site()),
                arguments: syn::PathArguments::None,
            },
            syn::PathSegment {
                ident: Ident::new("CopperRuntime", Span::call_site()),
                arguments: syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Token![<](Span::call_site()),
                        args: vec![syn::GenericArgument::Type(Type::Tuple(syn::TypeTuple {
                            paren_token: syn::token::Paren {
                                span: syn::token::Paren(Span::call_site()).span,
                            },
                            elems: all_configs_types.clone().into_iter().collect(),
                        }))]
                        .into_iter()
                        .collect(),
                        gt_token: Token![>](Span::call_site()),
                    },
                ),
            },
        ]
        .into_iter()
        .collect(),
    };

    // Create the new field
    let new_field = Field {
        attrs: Vec::new(),
        vis: Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(Ident::new("copper_runtime", Span::call_site())),
        colon_token: Some(Token![:](Span::call_site())),
        ty: Type::Path(syn::TypePath {
            qself: None,
            path: copper_runtime_path,
        }),
    };

    let name = &item_struct.ident;

    match &mut item_struct.fields {
        Named(fields_named) => {
            fields_named.named.push(new_field);
        }
        Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.push(new_field);
        }
        Unit => {
            // Handle unit structs if necessary
        }
    };

    // Generate the code to create instances of the nodes
    // It maps the types to their index
    let task_instances_init_code: Vec<_> = all_configs_types
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            let ty_name = &all_configs_type_names[index];
            let error = format!(
                "Failed to get create instance for {}, instance index {}.",
                ty_name, index
            );
            quote! {
                #ty::new(all_instances_configs[#index]).expect(#error)
            }
        })
        .collect();

    // Convert the modified struct back into a TokenStream
    let result = quote! {
        use copper::CuResult;
        use copper::config::ConfigNode;
        use copper::config::CopperConfig;
        use copper::config::NodeInstanceConfig;
        use copper::cutask::CuSrcTask; // Needed for the instantiation of tasks
        use copper::cutask::CuTask; // Needed for the instantiation of tasks
        use copper::cutask::CuSinkTask; // Needed for the instantiation of tasks
        use copper::runtime::CopperRuntime;
        use std::fs::read_to_string;

        pub #item_struct

        impl #name {

            pub fn new() -> CuResult<Self> {
                let config_filename = #config_file;

                let config_content = read_to_string(config_filename)
                    .unwrap_or_else(|_| panic!("Failed to read configuration file: {:?}", &config_filename));
                let copper_config = CopperConfig::deserialize(&config_content);
                let all_instances_configs: Vec<Option<&NodeInstanceConfig>>  = copper_config.get_all_nodes().iter().map(|node_config| node_config.get_instance_config()).collect();

                let tasks_instances = (
                    #(#task_instances_init_code),*
                );
                let copper_runtime = CopperRuntime::new(copper_config, tasks_instances);
                Ok(#name {
                    copper_runtime,
                })
            }
        }
    };

    let tokens: TokenStream = result.into();

    // Print and format the generated code using rustfmt
    println!("Generated tokens: {}", tokens);
    let formatted_code = rustfmt_generated_code(tokens.to_string());
    println!("\n     ===    Gen. Runtime ===\n");
    println!("{}", highlight_rust_code(formatted_code));
    println!("\n     === === === === === ===\n");

    tokens
}
