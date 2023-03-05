#![deny(clippy::all)]

struct Size {
    width: u32,
    height: u32,
}

enum Shapes {
    Circle { radius: u32, center: (i32, i32) },
    Triangle { base: u32, height: u32 },
    Rectangle { dimensions: Size },
    Point(i32, i32, i32),
}

fn main() {
    let circle = Shapes::Circle {
        radius: 20,
        center: (20, 20),
    };
    let triangle = Shapes::Triangle {
        base: 10,
        height: 20,
    };
    let rect = Shapes::Rectangle {
        dimensions: Size {
            width: 20,
            height: 30,
        },
    };
    let point = Shapes::Point(2, 3, 4);

    match circle {
        Shapes::Circle {
            radius: 20,
            center: (20, 20),
        } => {
            println!("Circle Match")
        }
        _ => println!("Not Covered"),
    }
}
