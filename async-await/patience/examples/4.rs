#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    println!("Hello, world!");

    let x = foo2();
}

async fn foo1() -> usize {
    println!("foo");
    0
}

// Future trait signifies the value is not ready yet
// and eventually be the value of usize.
fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo1");
        foo1().await;
        println!("foo2");
        0
    }
}

// Mental Model
// let x = read_to_string("file1").await;
//
// let fut = read_to_string("file1");
// let x = loop {
//   if let Some(result) = fut.try_check_completed() {
//     break result;
//   } else {
//     fut.try_make_progress();
//     yield;
//   }
// }
