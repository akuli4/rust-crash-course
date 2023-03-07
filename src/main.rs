#![deny(clippy::all)]

use std::collections::HashMap;

struct User {
    id: i128,
    firs_name: String,
    family_name: String,
}
fn main() {
    // Hash maps are not enabled by default, we need to import them.
    let mut users: HashMap<i128, User> = HashMap::new();

    users.insert(
        0,
        User {
            id: 0,
            firs_name: "Anna".to_string(),
            family_name: "Karenina".to_string(),
        },
    );

    let first_user = users.get(&0);
}
