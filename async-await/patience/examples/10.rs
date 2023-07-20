#![allow(dead_code, unused_variables)]

use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080").await;

    while let Ok(stream) = accept.await {
        // create another future
        // two threads can run parellel if the executor has more than 1 thread and the first thread is busy.
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(_: TcpStream) {
    todo!()
}
