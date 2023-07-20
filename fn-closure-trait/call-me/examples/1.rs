fn main() {
    println!("Hello, world!");

    // It looks like a function pointer, but it's not.
    // Type of x here is function item which is zero size value
    // It's a reference to a unique function bar
    let x = bar;
}

fn bar() {}
