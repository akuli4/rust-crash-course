#![deny(clippy::all)]

fn main() {
    /*
       Iterators are lazy, they need to be consumed before can be user.
       In rust a for loop automatically consumes and iterator.
    */
    let values = vec![2, 4, 2, 1];
    for value in values.iter() {
        println!("{}", value);
    }
}
