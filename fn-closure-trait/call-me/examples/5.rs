fn main() {
    println!("Hello, world!");

    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    baz(bar::<u32>);
    baz(bar::<i32>);
    quox(bar::<u32>);
}

fn bar<T>(_: u32) -> u32 {
    0
}

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f));
}

fn quox<F>(f: F)
where
    F: Fn(), // Implement Fn Trait
{
}

// Ref: https://doc.rust-lang.org/std/ops/trait.FnOnce.html
// Since both Fn and FnMut are subtraits of FnOnce, any instance of Fn or FnMut can be used where a FnOnce is expected.

impl<F> FnOnce() for F
where
    F: Fn(),
{
    fn call(self) {
        Fn::cal(&self);
    }
}

impl<F> FnOnce() for F
where
    F: FnMut(),
{
    fn call(&mut self) {
        FnMut::cal(&mut self);
    }
}

impl<F> Fn() for F
where
    F: FnMut(),
{
    fn call(&mut self) {
        Fn::cal(&*self);
    }
}
