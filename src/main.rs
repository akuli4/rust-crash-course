#![deny(clippy::all)]

fn main() {
    /*
       Iterators are lazy, they need to be consumed before can be user.
       In rust a for loop automatically consumes and iterator.

       Iterators cannot be double-consumed.

       The pointer is at the end after first iteration and stays there.
    */
    let values = vec![2, 4, 2, 1];
    let iter = values.iter();
    let sum1: i32 = iter.sum();
    let sum2: i32 = iter.sum(); // use of a moved value
}
