fn main() {
    println!("Hello, world!");

    let mut z = String::new();
    // captured closure
    let mut f = || {
        z.clear();
    };

    let f: &mut dyn FnMut() = &mut f;
    quox(f);
}

fn quox<F>(mut f: F)
where
    F: FnMut(),
{
    (f)()
}
