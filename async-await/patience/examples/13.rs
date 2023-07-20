#![allow(dead_code, unused_variables)]

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() {
    let x = foo();
}

// To keep the state
// enum StateMachine {
//   Chunk1 {
//     buffer: [u8, 1024],
//     fut: tokio::fs::File<'x>,
//   },
//   Chunk2 {},
// }

async fn foo() -> io::Result<()> {
    let mut buffer = [0; 1024];

    // chuck1
    let mut f = File::open("foo.txt").await?;
    // fut.await
    // yield (really: return); buffer is go away

    // chuck2
    let n = f.read(&mut buffer[..]).await?;
    // fut.await

    println!("Read bytes: {}", n);

    Ok(())
}
