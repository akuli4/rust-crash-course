#![deny(clippy::all)]

fn main() {
    let tuple = (2, 2.3, "Hi");
    // tuples are heteregeneous, meaning they are of diverse in character or content.
    // destructuring
    let (_, _, string) = tuple;
}
