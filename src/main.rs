#![deny(clippy::all)]

fn main() {
    struct User {
        first_name: String,
        family_name: String,
        picture_url: String,
    }
    impl User {
        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.family_name)
        }
    }
    let user1 = User {
        first_name: "Darth".to_string(),
        family_name: "Vader".to_string(),
        picture_url: "https://akamai.darth-vader-pics.bruh/a29aDMas9dsl".to_string(),
    };

    println!(
        "
            first_name: {},
            family_name: {},
            picture_url: {}
            full_name: {}
        ",
        user1.first_name,
        user1.family_name,
        user1.picture_url,
        user1.full_name()
    );
}
