fn main() {
    println!("Hello, world!");

    let mut x = bar::<i32>;
    println!("bar size: {}", std::mem::size_of_val(&x));

    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(bar::<u32>);

    // non-capture closure
    // f is a function item
    let f = |x: i32, y: i32| x + y;
    println!("f size: {}", std::mem::size_of_val(&f)); // size 0

    // non-captured closure can converse to function pointer
    let f = || ();
    baz(f);
    quox(f);
    quox(&f);

    let z = String::new(); // non-copy value

    // captured closure
    let f = || {
        println!("{}", z);
    };

    // Think of it like the compiler generates
    // struct FClosure<'scope> {
    //   z: &'scope String
    // }
    // impl<'scope> Fn() for FClosure<'scope> {
    //   fn call(&self) {
    //     // copy-paste from closure definition
    //     println!("{}", self.z);
    //   }
    // }

    baz(f); // not working
    quox(&f); // working
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
