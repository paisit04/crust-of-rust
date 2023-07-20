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

    let z = String::new();

    // captured closure
    let f = || {
        println!("{}", z);
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

fn hello(f: Box<dyn Fn()>) {
    f();
}

fn hello2(mut f: Box<dyn FnMut()>) {
    f();
}

fn hello3(f: Box<dyn FnOnce()>) {
    f();
}

// It used to be
// Box<dyn Fn()> did not implement Fn() the same with FnMut and FnOnce
// impl FnOnce() for Box<dyn FnOnce()> {
//     fn call(self) {
//         let x: dyn FnOnce() = self.0; // This type is not Sized
//         x.call(); // what is the type of self.0?
//     }
// }
// Ref: https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html#fn-closure-traits-implemented-for-boxdyn-fn
// This was ultimately due to a limitation in the compiler's ability to reason about such implementations, which has since been fixed with the introduction of unsized locals.
