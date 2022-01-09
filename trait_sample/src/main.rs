use crate::geo::Geometry;

mod geo;

fn main() {
    let a = &geo::Rectangle {
        width: 10,
        height: 20,
    };
    let b = &geo::Triangle {
        bottom: 20,
        height: 5,
    };

    dbg!(a);
    dbg!(b);

    println!("{}, {}", a.name(), a.area());
    println!("{}, {}", b.name(), b.area());
}
