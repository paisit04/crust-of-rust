fn main() {
    quox(|x| x);
}

// trait bound - for syntax
// for any lifetime 'a
// help Fn to declare a lifetime
fn quox<F>(f: F)
where
    // F: for<'a> Fn(&'a str) -> &'a str,
    F: Fn(&str) -> &str,
{
}
