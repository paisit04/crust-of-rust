fn main() {
    // no for loop in rust
    for x in vec!["a", "b", "c"] {}

    // turns into
    let mut iter = vec!["a", "b", "c"].into_iter();
    while let Some(x) = iter.next() {}

    // Ref: https://doc.rust-lang.org/std/iter/trait.IntoIterator.html
}

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
//
// trait Iterator<Item> {
//     fn next(&mut self) -> Option<Item>;
// }
//
// In general, you use an associated type
// if you expect that there is one implementation of a trait for a given time.
//
// You might want a generic type
// if you expect that there are many implementations of a trait for a given time.
//
// trait Service<Request> {
//     fn do(&mut self, r: Request);
// }
