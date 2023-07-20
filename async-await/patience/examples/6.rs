#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    let mut terminal = read_from_terminal();

    let mut foo = foo2();

    loop {
        select! {
            foo <- (&mut foo).await => {}
            line <-(&mut terminal).await => {
                // do something with line
            }
        }
    }
}

async fn read_from_terminal() {}

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
