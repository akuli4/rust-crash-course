#![deny(clippy::all)]

fn main() {
    let vec2 = vec![4, 5, 6];

    if vec2.contains(&4) {
        println!("contains 4")
    };
}
