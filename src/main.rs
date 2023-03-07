#![deny(clippy::all)]

fn main() {
    // Fix sized vector of string slices.
    let vector: [&str; 2] = ["foo", "bar"];

    for value in vector.iter() {
        println!("{}", value)
    }
}
