#![deny(clippy::all)]

enum Pet {
    Cat { name: String },
    Dog { name: String },
}
fn main() {
    let bobby = Pet::Cat {
        name: "Bobby".to_string(),
    };
    // How to extract pets name?
    // match is an expression
    let name = match bobby {
        Pet::Cat { name } => name,
        Pet::Dog { name } => name,
    };

    println!("Name is {}", name);
}
