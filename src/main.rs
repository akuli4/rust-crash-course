#![deny(clippy::all)]

fn main() {
    let tuple = get_values();
    // tuples are heteregeneous, meaning they are of diverse in character or content.
    // destructuring
    let (_, _, string) = tuple;

    fn get_values() -> (i32, f32, String) {
        (2, 2.3, String::from("Hello"))
    }
}
