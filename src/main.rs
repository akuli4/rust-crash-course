#![deny(clippy::all)]

use std::collections::HashMap;
fn main() {
    // Hash maps are not enabled by default, we need to import them.
    let mut user: HashMap<&str, &str> = HashMap::new();
    user.insert("foo", "bar");
    user.insert("boo", "far");

    let entry = user.entry("foo"); // entry.key(), entry.into()

    match entry {
        std::collections::hash_map::Entry::Occupied(mut o) => {
            o.get_mut();
        }
        _ => {}
    }
}
