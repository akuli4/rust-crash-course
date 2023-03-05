#![deny(clippy::all)]

fn main() {
    struct Point(f32, f32, f32);
    impl Point {
        fn twice(&self) -> Point {
            Point(self.0 * 2.0, self.1 * 2.0, self.2 * 2.0)
        }
    }

    let x = Point(2.0, 10.0, 3.0);
    let y = x.twice();

    println!(
        "Point: ({}, {}, {}), twice call: {} {} {}",
        x.0, x.1, x.2, y.0, y.1, y.2
    );
}
