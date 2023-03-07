#![deny(clippy::all)]

fn main() {
    // Fix sized vector of string slices.
    let vector: [i32; 2] = [2, 3];

    let mapped = vector.iter().map(|x| x * 2);
}
