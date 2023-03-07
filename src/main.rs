#![deny(clippy::all)]

fn main() {
    // Dynamic sized vector of string slices.
    let vector = vec![2, 3];
    //or
    let mut vector = Vec::new();

    vector.push(42);
    vector.push(32);
    vector.push(10);

    let mapped = vector.iter().map(|x| x * 2);
}
