#![deny(clippy::all)]

fn say_anything(string: &str) {
    println!("{}", string)
}

fn main() {
    say_anything("Hello World")
}
