macro_rules! foo {
    ($x:expr) => {
        println!("{}", $x)
    };
}
fn main() {
    foo!(1 + 2);
}
