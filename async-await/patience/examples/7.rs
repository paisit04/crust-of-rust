#![allow(dead_code, unused_variables)]

use std::future::Future;

fn main() {
    let terminal = read_from_terminal();

    let mut foo = foo2();

    loop {
        select! {
            foo <- foo.await => {}
            line <- terminal.await => {
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

// Cancel Case
fn foo2(cancal: tokio::sync::mpsc::Receiver<()>) -> impl Future<Output = usize> {
    async {
        println!("foo1");
        race! {
            done <- foo1().await => {

            }
            cancel <- cancel.await => {
                return 0;
            }
        }

        println!("foo2");
        0
    }
}
