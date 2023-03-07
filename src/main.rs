#![deny(clippy::all)]

fn main() {
    // Dynamic sized vector of string slices.
    let vector = vec![2, 3];
    //or
    let mut vector = Vec::new();

    vector.extend_from_slice(&[2, 10, 20]);

    let mapped = vector.iter().map(|x| x * 2);
}
