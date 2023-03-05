#![deny(clippy::all)]

struct Size {
    width: u32,
    height: u32,
}

enum Shapes {
    Circle { radius: u32, center: (i32, i32) },
    Rectangle(u32, u32, Size),
}

fn main() {
    let rect = Shapes::Rectangle(
        20,
        21,
        Size {
            width: 2,
            height: 3,
        },
    );

    if let Shapes::Rectangle(x, y, Size { width, height }) = rect {
        println!("Match! {} {} {} {}", x, y, width, height)
    };
}
