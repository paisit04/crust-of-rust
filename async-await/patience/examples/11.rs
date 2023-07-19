#![allow(dead_code, unused_variables)]

use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080").await;

    while let Ok(stream) = accept.await {
        // create another future
        // two threads can run parellel if the executore has more than 1 thread and the first thread is busy.
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(_: TcpStream) {
    // use the same technique as multithreading
    // or use channel to communicate
    let x = Arc::new(Mutex::new(vec![]));

    let x1 = Arc::clone(&x);
    tokio::spawn(async move {
        x1.lock();
    });

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock();
    });
}
