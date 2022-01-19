use attribute_macro::trace;

fn main() {
    test()
}

#[trace]
fn test() {
    println!("Hello, world!");
}
