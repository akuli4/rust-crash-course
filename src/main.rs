#![deny(clippy::all)]

struct Size {
    width: u32,
    height: u32,
}

enum Shapes {
    Rectangle(u32, u32, Size),
}

impl Shapes {
    fn area(&self) -> u32 {
        match self {
            Shapes::Rectangle(_x, _y, size) => size.width * size.height,
        }
    }
}

fn main() {
    let rect = Shapes::Rectangle(
        20,
        21,
        Size {
            width: 5,
            height: 7,
        },
    );

    println!("{}", format_args!("{}", rect.area()));
}
