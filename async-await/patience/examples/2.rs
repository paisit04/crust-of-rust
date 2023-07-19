#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let x = foo1(); // x is not usize
}

async fn foo1() -> usize {
    0
}

// Future trait signifies the value is not ready yet
// and eventually be the value of usize.
fn foo2() -> impl Future<Output = usize> {
    async { 0 }
}
