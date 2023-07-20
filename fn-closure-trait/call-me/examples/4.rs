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

// Fn takes &T, you can call it multiple times and call multiple threads at the same time
//       through the shared reference
// FnMut takes &mut T, call Once at a time and can call it multiple times
//       cannot call multiple threads at the same time.
// FnOnce takes T, call FnOnce only single time
fn quox<F>(f: F)
where
    F: Fn(), // Implement Fn Trait
{
}

// fn() function pointer has no state, no reference, no lifetime
// means don't care about self (pointer to the function)
// function pointer implement all three (Fn, FnMut, FnOnce)
