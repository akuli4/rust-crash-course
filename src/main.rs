#![deny(clippy::all)]

#[derive(PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn main() {
    // Enums are useful when it comes to related objects.
    let direction = Direction::East;

    match direction {
        Direction::North => println!("Heading North"),
        // Default case
        _ => println!("Where am I?"),
    }
}
