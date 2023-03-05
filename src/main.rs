#![deny(clippy::all)]

fn main() {
    let multiply_by_2 = |num: i32| num * 2;

    println!("{}", multiply_by_2(2))
}
