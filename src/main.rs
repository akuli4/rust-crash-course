#![deny(clippy::all)]

fn main() {
    let calculate_bmi = |weight: f32, height: f32| weight / height.powf(2.0);

    println!("Your bmi is - {:.2}", calculate_bmi(70.0, 1.86));
}
