fn main() {
    let vs = vec![1, 2, 3];

    for v in vs {
        // consumes vs, owned v
        // `vs` moved due to this implicit call to `.into_iter()
    }

    for v in vs.iter() {
        // borrows vs, & to v
    }

    for v in &vs {
        // equivalent to vs.iter()
    }
}
