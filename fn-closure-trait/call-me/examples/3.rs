fn main() {
    println!("Hello, world!");

    let mut x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x)); // 0 bytes

    // the complier converses the function item type to function pointer type
    // you cannot converse function pointer to function item
    baz(bar::<u32>);
    baz(bar::<i32>);
}

fn bar<T>(_: u32) -> u32 {
    0
}

// baz is take a function pointer which a function takes u32 and returns u32
// it's generic as long as the function has the same signature.
fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f)); // 8 bytes
}
