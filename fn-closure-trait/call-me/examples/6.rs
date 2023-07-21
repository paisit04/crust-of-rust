fn main() {
    println!("Hello, world!");

    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(bar::<u32>);
    quox2(bar::<u32>);
    quox3(bar::<u32>);
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}

fn quox<F>(f: F)
where
    F: Fn(),
{
    (f)()
}

fn quox2<F>(mut f: F)
where
    F: FnMut(),
{
    (f)()
}

fn quox3<F>(f: F)
where
    F: FnOnce(),
{
    (f)()
}
