fn main() {
    println!("Hello, world!");

    let z = String::new();
    // captured closure
    let f = || {
        println!("{}", z);
    };

    let f: &dyn Fn() = &f;
    quox(f);
}

fn quox<F>(mut f: F)
where
    F: FnMut(),
{
    (f)()
}
