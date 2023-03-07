#![deny(clippy::all)]

fn main() {
    /*
       Iterators are lazy, they need to be consumed before can be user.
       In rust a for loop automatically consumes and iterator.

       Iterators cannot be double-consumed.

       The pointer is at the end after first iteration and stays there.

       .iter() allows you to work with references from an itrable.
    */
    let values = vec![2, 4, 2, 1];

    for value in values.into_iter() {
		
	}
}
