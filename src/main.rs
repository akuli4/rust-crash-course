#![deny(clippy::all)]

fn main() {
    struct Point(f32, f32, f32);
    impl Point {
        fn double(&mut self) {
            self.0 *= 2.0;
            self.1 *= 2.0;
            self.2 *= 2.0;
        }
    }

    let mut x = Point(2.0, 10.0, 3.0);
    x.double();

    println!("Point: ({}, {}, {})", x.0, x.1, x.2);
}
