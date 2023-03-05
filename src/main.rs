#![deny(clippy::all)]

#[derive(PartialEq)]
enum Shapes {
    Circle { radius: u32, center: (i32, i32) },
    Triangle { base: u32, height: u32 },
    Rectangle { left_side: u32, right_side: u32 },
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
    let rectangle = Shapes::Rectangle {
        left_side: 6,
        right_side: 8,
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
