#![deny(clippy::all)]

fn main() {
    let say_hello = |name: &str| format!("Hello, {}", name);

    println!("{}", say_hello("Alan"));
}
