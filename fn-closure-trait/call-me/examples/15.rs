fn main() {
    println!("Hello, world!");

    let mut z = String::new();
    // captured closure
    let f = || {
        println!("{}", z);
    };

    let f: Box<dyn FnOnce()> = Box::new(f);
    quox(f);
}

fn quox<F>(f: F)
where
    F: FnOnce(),
{
    (f)()
}
