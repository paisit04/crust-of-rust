fn main() {
    println!("Hello, world!");

    let z = String::new();
    // captured closure
    let f = || {
        println!("{}", z);
    };

    // change to FnMut
    let f: &dyn FnMut() = &f;
    quox(f); // the trait `Fn<()>` is not implemented for `dyn FnMut()`
}

fn quox<F>(mut f: F)
where
    F: FnMut(),
{
    (f)()
}
