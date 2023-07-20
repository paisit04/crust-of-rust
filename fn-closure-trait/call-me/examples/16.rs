fn main() {
    println!("Hello, world!");

    // This is a constant closure
    // You can evaluate the closure at compile time
    // which similar to make_zero()
    let x = || 0;
}

const fn make_zero() -> i32 {
    0
}

// The compiler does not know where F is callable at the compile time
// We didn't the implementation of FnOnce whether it is a const.
const fn foo<F: FnOnce()>(f: F) {
    f();
}

// Ref: https://github.com/rust-lang/rust/issues/67792
// Experiment Feature
