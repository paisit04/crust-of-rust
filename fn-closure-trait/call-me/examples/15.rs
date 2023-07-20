fn main() {
    println!("Hello, world!");

    let mut z = String::new();
    // captured closure
    let f = || {
        println!("{}", z);
    };

    let f: Box<dyn FnOnce()> = Box::new(f);
    quox(f); // the trait `Fn<()>` is not implemented for `dyn FnMut()`
}

fn quox<F>(f: F)
where
    F: FnOnce(),
{
    (f)()
}
