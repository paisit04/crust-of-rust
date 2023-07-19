#![allow(dead_code)]

use std::future::Future;

fn main() {
    println!("Hello, world!");
}

// foo1 and foo2 are the same.
// async keyword is a transformation directive for the complier.
async fn foo1() {}

fn foo2() -> impl Future<Output = ()> {
    async {}
}
