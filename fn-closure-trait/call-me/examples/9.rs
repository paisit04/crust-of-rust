fn main() {
    println!("Hello, world!");

    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(bar::<u32>);

    // non-capture closure
    let f = |x: i32, y: i32| x + y;

    // non-captured closure can converse to function pointer
    let f = || ();
    baz(f);
    quox(f);
    quox(&f);

    // change to mut
    let mut z = String::new();

    // captured closure
    let f = || {
        // the complier know that we need to move z to the closure
        drop(z); // We need the ownership of z
    };

    // Think of it like the complier generates
    // struct FClosure<'scope> {
    //   z: String
    // }
    // impl<'scope> FnOnce() for FClosure<'scope> {
    //   fn call(self) {
    //     // copy-paste from closure definition
    //     drop(self.z);
    //   }
    // }

    // baz(f); // not working
    // quox(&f); // not working
    quox3(f);
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

fn quox3<F>(f: F)
where
    F: FnOnce(),
{
    (f)()
}
