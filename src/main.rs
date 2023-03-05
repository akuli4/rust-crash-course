#![deny(clippy::all)]

fn main() {
    #[derive(Debug)]
    struct Point(f32, f32, f32);

    impl Point {
        // Non method associated function
        fn zero() -> Point {
            Point(0.0, 0.0, 0.0)
        }
    }

    impl Point {
        // fn twice(&self) -> Point {
        //     Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
        // }
    }

    let x = Point::zero();
    let y = Point::zero();
    let z = Point::zero();

    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
}
