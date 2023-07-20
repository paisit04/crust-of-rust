fn main() {
    println!("Hello, world!");

    // x is not a function pointer
    // x is a unique identifier of a function item
    let mut x = bar::<i32>;

    println!("{}", std::mem::size_of_val(&x)); // 0 size

    // x = bar::<u32>; // mismatched type
}

fn bar<T>() {}
