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
        z.clear();
    };

    // Think of it like the complier generates
    // struct FClosure<'scope> {
    //   z: &'scope mut String
    // }
    // impl<'scope> FnMut() for FClosure<'scope> {
    //   fn call(self) { // it's shared reference
    //     // copy-paste from closure definition
    //     self.z.clear(); // it's not working because we need to exclusive reference
    //     & &mut String -> &String // it's a shared reference to the mut ref
    //   }
    // }

    baz(f); // not working
    quox(&f); // not working
    quox2(f); // working
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
