mod format;
#[cfg(feature = "macro_debug")]
use format::{highlight_rust_code, rustfmt_generated_code};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Build a fixed sized SoA (Structure of Arrays) from a struct.
/// The outputted SoA will be suitable for in place storage in messages and should be
/// easier for the compiler to vectorize.
///
/// for example:
///
/// ```ignore
/// #[derive(Soa)]
/// struct MyStruct {
///    a: i32,
///    b: f32,
/// }
/// ```
///
/// will generate:
/// ```ignore
/// pub struct MyStructSoa<const N: usize> {
///     pub a: [i32; N],
///     pub b: [f32; N],
/// }
/// ```
///
/// You can then use the generated struct to store multiple
/// instances of the original struct in an SoA format.
///
/// ```ignore
/// // makes an SOA with a default value
/// let soa1: MyStructSoa<8> = XyzSoa::new(MyStruct{ a: 1, b: 2.3 });
/// ```
///
/// Then you can access the fields of the SoA as slices:
/// ```ignore
/// let a = soa1.a();
/// let b = soa1.b();
/// ```
///
/// You can also access a range of the fields:
/// ```ignore
/// let a = soa1.a_range(0..4);
/// let b = soa1.b_range(0..4);
/// ```
///
/// You can also modify the fields of the SoA:
/// ```ignore
/// soa1.a_mut()[0] = 42;
/// soa1.b_mut()[0] = 42.0;
/// ```
///
/// You can also modify a range of the fields:
/// ```ignore
/// soa1.a_range_mut(0..4)[0] = 42;
/// soa1.b_range_mut(0..4)[0] = 42.0;
/// ```
///
/// You can also apply a function to all the fields of the SoA:
/// ```ignore
/// soa1.apply(|a, b| {
///    (a + 1, b + 1.0)
/// });
/// ```
// #[proc_macro_derive(Soa)]
// pub fn derive_soa(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);
//     let visibility = &input.vis;
//
//     let name = &input.ident;
//     let module_name = format_ident!("{}_soa", name.to_string().to_lowercase());
//     let soa_struct_name = format_ident!("{}Soa", name);
//
//     let data = match &input.data {
//         Data::Struct(data) => data,
//         _ => panic!("Only structs are supported"),
//     };
//     let fields = match &data.fields {
//         Fields::Named(fields) => &fields.named,
//         _ => panic!("Only named fields are supported"),
//     };
//
//     let mut field_names = vec![];
//     let mut field_names_mut = vec![];
//     let mut field_names_range = vec![];
//     let mut field_names_range_mut = vec![];
//     let mut field_types = vec![];
//     let mut unique_field_names = vec![];
//     let mut unique_field_types = vec![];
//
//     for field in fields {
//         let field_name = field.ident.as_ref().unwrap();
//         let field_type = &field.ty;
//         field_names.push(field_name);
//         field_types.push(field_type);
//
//         // find unique field types by name
//         let type_name = field_type.into_token_stream().to_string();
//         if !unique_field_names.contains(&type_name) {
//             unique_field_names.push(type_name);
//             unique_field_types.push(field_type);
//         }
//
//         field_names_mut.push(format_ident!("{}_mut", field_name));
//         field_names_range.push(format_ident!("{}_range", field_name));
//         field_names_range_mut.push(format_ident!("{}_range_mut", field_name));
//     }
//
//     let expanded = quote! {
//         #visibility mod #module_name {
//             use bincode::Decode;
//             use bincode::Encode;
//             use bincode::enc::Encoder;
//             use bincode::de::Decoder;
//             use bincode::error::DecodeError;
//             use bincode::error::EncodeError;
//
//             #[derive(Debug)]
//             #visibility struct #soa_struct_name<const N: usize> {
//                 #(pub #field_names: [#field_types; N]),*
//             }
//
//             impl<const N: usize> #soa_struct_name<N> {
//                 pub fn new(default: super::#name) -> Self {
//                     Self {
//                         #( #field_names: [default.#field_names; N] ),*
//                     }
//                 }
//
//                 pub fn set(&mut self, index: usize, value: super::#name) {
//                     assert!(index < N, "Index out of bounds");
//                     #( self.#field_names[index] = value.#field_names; )*
//                 }
//
//                 pub fn get(&self, index: usize) -> super::#name {
//                     assert!(index < N, "Index out of bounds");
//                     super::#name {
//                         #( #field_names: self.#field_names[index], )*
//                     }
//                 }
//
//                 pub fn apply<F>(&mut self, mut f: F)
//                 where
//                     F: FnMut(#(#field_types),*) -> (#(#field_types),*)
//                 {
//                     for i in 0..N {
//                         let result = f(#(self.#field_names[i]),*);
//                         let (#(#field_names),*) = result;
//                         #(
//                             self.#field_names[i] = #field_names;
//                         )*
//                     }
//                 }
//
//                 #(
//                     pub fn #field_names(&self) -> &[#field_types] {
//                         &self.#field_names
//                     }
//
//                     pub fn #field_names_mut(&mut self) -> &mut [#field_types] {
//                         &mut self.#field_names
//                     }
//
//                     pub fn #field_names_range(&self, range: std::ops::Range<usize>) -> &[#field_types] {
//                         &self.#field_names[range]
//                     }
//
//                     pub fn #field_names_range_mut(&mut self, range: std::ops::Range<usize>) -> &mut [#field_types] {
//                         &mut self.#field_names[range]
//                     }
//                 )*
//             }
//
//             impl<const N: usize> Encode for #soa_struct_name<N> {
//                 fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
//                     #(
//                         for i in 0..N {
//                             self.#field_names[i].encode(encoder)?;
//                         }
//                     )*
//                     Ok(())
//                 }
//             }
//
//             impl<const N: usize> Decode for #soa_struct_name<N> {
//                 fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
//                     let mut result = Self::default();
//                     #(
//                         for i in 0..N {
//                             result.#field_names[i] = Decode::decode(decoder)?;
//                         }
//                     )*
//                     Ok(result)
//                 }
//             }
//
//             impl<const N: usize> Default for #soa_struct_name<N> {
//                 fn default() -> Self {
//                     Self {
//                         #( #field_names: [#field_types::default(); N] ),*
//                     }
//                 }
//             }
//
//             impl<const N: usize> std::ops::Add for #soa_struct_name<N>
//             where
//                 #(
//                     #field_types: std::ops::Add<Output = #field_types> + Copy,
//                 )*
//             {
//                 type Output = Self;
//
//                 fn add(self, other: Self) -> Self {
//                     Self {
//                         #(
//                             #field_names: {
//                                 let mut result = [self.#field_names[0]; N];
//                                 result.iter_mut().zip(self.#field_names.iter().zip(other.#field_names.iter()))
//                                     .for_each(|(res, (a, b))| *res = *a + *b);
//                                 result
//                             }
//                         ),*
//                     }
//                 }
//             }
//
//             impl<const N: usize> std::ops::Sub for #soa_struct_name<N>
//             where
//                 #(
//                     #field_types: std::ops::Sub<Output = #field_types> + Copy,
//                 )*
//             {
//                 type Output = Self;
//
//                 fn sub(self, other: Self) -> Self {
//                     Self {
//                         #(
//                             #field_names: {
//                                 let mut result = [self.#field_names[0]; N];
//                                 result.iter_mut().zip(self.#field_names.iter().zip(other.#field_names.iter()))
//                                     .for_each(|(res, (a, b))| *res = *a - *b);
//                                 result
//                             }
//                         ),*
//                     }
//                 }
//             }
//
//             impl<const N: usize> Clone for #soa_struct_name<N>
//             where
//                 #(
//                     #field_types: Clone,
//                 )*
//             {
//                 fn clone(&self) -> Self {
//                     Self {
//                         #( #field_names: self.#field_names.clone(), )*
//                     }
//                 }
//             }
//         }
//         #visibility use #module_name::#soa_struct_name;
//     };
//
//     let tokens: TokenStream = expanded.into();
//
//     #[cfg(feature = "macro_debug")]
//     {
//         let formatted_code = rustfmt_generated_code(tokens.to_string());
//         eprintln!("\n     ===    Gen. SOA     ===\n");
//         eprintln!("{}", highlight_rust_code(formatted_code));
//         eprintln!("\n     === === === === === ===\n");
//     }
//     tokens
// }

#[proc_macro_derive(Soa)]
pub fn derive_soa(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let visibility = &input.vis;

    let name = &input.ident;
    let module_name = format_ident!("{}_soa", name.to_string().to_lowercase());
    let soa_struct_name = format_ident!("{}Soa", name);

    let data = match &input.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };
    let fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => panic!("Only named fields are supported"),
    };

    let mut flat_field_decls = vec![];
    let mut grouped_field_decls = vec![];
    let mut flat_field_methods = vec![];
    let mut grouped_field_methods = vec![];
    let mut new_initializations = vec![];
    let mut default_initializations = vec![];

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        let field_name_mut = format_ident!("{}_mut", field_name);

        let field_type_str = field_type.into_token_stream().to_string();
        if field_type_str.ends_with("Soa") {
            flat_field_decls.push(quote! { pub #field_name: [#field_type; N] });
            new_initializations.push(quote! { #field_name: [default.#field_name; N] });
            default_initializations.push(quote! { #field_name: [Default::default(); N] });
            flat_field_methods.push(quote! {
                pub fn #field_name(&self) -> &[#field_type] {
                    &self.#field_name
                }
                pub fn #field_name_mut(&mut self) -> &mut [#field_type] {
                    &mut self.#field_name
                }
            });
        } else {
            grouped_field_decls.push(quote! { pub #field_name: [#field_type; N] });
            new_initializations.push(quote! { #field_name: [default.#field_name.clone(); N] });
            default_initializations.push(quote! { #field_name: [Default::default(); N] });
            grouped_field_methods.push(quote! {
                pub fn #field_name(&self) -> &[#field_type; N] {
                    &self.#field_name
                }
                pub fn #field_name_mut(&mut self) -> &mut [#field_type; N] {
                    &mut self.#field_name
                }
            });
        }
    }

    let expanded = quote! {
        #visibility mod #module_name {
            use bincode::{Decode, Encode};
            use bincode::enc::Encoder;
            use bincode::de::Decoder;
            use bincode::error::{DecodeError, EncodeError};

            #[derive(Debug)]
            #visibility struct #soa_struct_name<const N: usize> {
                #(#flat_field_decls,)*
                #(#grouped_field_decls,)*
            }

            impl<const N: usize> #soa_struct_name<N> {
                pub fn new(default: super::#name) -> Self {
                    Self {
                        #(#new_initializations),*
                    }
                }

                // Accessors for flattened fields
                #(#flat_field_methods)*

                // Accessors for grouped fields
                #(#grouped_field_methods)*
            }

            impl<const N: usize> Encode for #soa_struct_name<N> {
                fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
                    Ok(())
                }
            }

            impl<const N: usize> Decode for #soa_struct_name<N> {
                fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
                    let mut result = Self::default();
                    Ok(result)
                }
            }

            impl<const N: usize> Default for #soa_struct_name<N> {
                fn default() -> Self {
                    Self {
                        #(#default_initializations),*
                    }
                }
            }
        }

        #visibility use #module_name::#soa_struct_name;
    };

    let tokens: TokenStream = expanded.into();

    #[cfg(feature = "macro_debug")]
    {
        let formatted_code = rustfmt_generated_code(tokens.to_string());
        eprintln!("\n     ===    Gen. SOA     ===\n");
        eprintln!("{}", highlight_rust_code(formatted_code));
        eprintln!("\n     === === === === === ===\n");
    }
    tokens
}
