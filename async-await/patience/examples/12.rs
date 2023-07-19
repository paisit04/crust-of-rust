#![allow(dead_code, unused_variables)]

use std::sync::Arc;
use std::sync::Mutex;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080").await;

    while let Ok(stream) = accept.await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(_: TcpStream) {
    let x = Arc::new(Mutex::new(vec![]));

    let x1 = Arc::clone(&x);
    // spawn a future
    let join_handle = tokio::spawn(async move {
        x1.lock();
        0
    });

    assert_eq!(join_handle.await, 0);

    let x2 = Arc::clone(&x);
    tokio::spawn(async move {
        x2.lock();

        // What you do with the error
        // You have no way to communicate it
        // e.g. spawn a thread and the error happens in main
        // Suggest! should not propagate errors further
        // Using tracing or logging
        let x: Result<_, _> = definitely_errors();
    });
}
