fn main() {
    println!("Hello, world!");

    let mut z = String::new();
    // captured closure
    let mut f = || {
        z.clear();
    };

    let f: &mut dyn FnMut() = &mut f;
    quox(f); // the trait `Fn<()>` is not implemented for `dyn FnMut()`
}

fn quox<F>(mut f: F)
where
    F: FnMut(),
{
    (f)()
}
