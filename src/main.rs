#![deny(clippy::all)]

#[derive(PartialEq)]
enum Shapes {
    Circle { radius: u32 },
    Triangle { base: u32, height: u32 },
    Rectangle { left_side: u32, right_side: u32 },
}

fn main() {
    let circle = Shapes::Circle { radius: 20 };
    let triangle = Shapes::Triangle {
        base: 10,
        height: 20,
    };
    let rectangle = Shapes::Rectangle {
        left_side: 6,
        right_side: 8,
    };

    if let Shapes::Circle { radius: 20 } = circle {
        println!("Circle Match")
    }

    if let Shapes::Triangle {
        base: 10,
        height: 12,
    } = triangle
    {
        println!("Tringle Match")
    }

    if let Shapes::Rectangle {
        left_side: 6,
        right_side: 8,
    } = rectangle
    {
        println!("React Match")
    }
}
