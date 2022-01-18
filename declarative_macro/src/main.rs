macro_rules! foo {
    ($x:expr) => {
        println!("{}", $x)
    };
}

macro_rules! array {
    ($ ($x:expr), *) => {
        [ $( $x), *]
    };
}
fn main() {
    foo!(1 + 2);

    let ar = array!(1, 2, 3, 4);
    for v in ar.iter() {
        println!("{}", v);
    }
}
