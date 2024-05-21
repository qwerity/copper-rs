use copper_derive::copper_runtime;

#[copper_runtime(config = "copperconfig.ron")]
struct MyApplication {}

fn main() {
    let app = MyApplication::new().expect("Failed to create runtime.");
}
