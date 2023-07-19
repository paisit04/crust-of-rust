#![allow(dead_code, unused_variables)]

use std::future::Future;

async fn foo1() -> usize {
    println!("foo");
    0
}

// Future trait signifies the value is not ready yet
// and eventually be the value of usize.
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo");
        0
    }
}

fn main() {
    println!("Hello, world!");

    // This will not print foo.
    // The future does nothing until it does await
    // The future just describes the series of steps that will be executes at some point in the future
    let x = foo1();
}
