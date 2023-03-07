#![deny(clippy::all)]

fn main() {
    let vec2 = vec![4, 5, 6];

    if !vec2.is_empty() {
        println!("vec2 isn't empty")
    };
}
