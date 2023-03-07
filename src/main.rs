#![deny(clippy::all)]

use std::collections::HashMap;
fn main() {
    // Hash maps are not enabled by default, we need to import them.
    let mut user: HashMap<&str, &str> = HashMap::new();
    user.insert("first_name", "Anna");
    user.entry("family_name").or_insert("Karenina");
}
