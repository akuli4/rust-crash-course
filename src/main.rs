#![deny(clippy::all)]

fn generate_hello_statement(string: &str) -> String {
    format!("Hi, {}!", string)
}

fn main() {
    let bar = generate_hello_statement("Alan");

    println!("{}", bar)
}
