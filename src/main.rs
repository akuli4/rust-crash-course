#![deny(clippy::all)]

fn multiply(num1: i32, num2: i32) -> i32 {
    num1 * num2
}
//  (-> ()) === (=> void)

fn main() {
    let num1 = 2;
    let num2 = 3;
    println!(
        "The product of {0} and {1} is {2}",
        num1,
        num2,
        multiply(num1, num2)
    );
}
