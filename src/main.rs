#![deny(clippy::all)]

fn main() {
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    // Move values from vec2 to vec1
    vec1.append(&mut vec2);

    // vec2 is empty now
}
