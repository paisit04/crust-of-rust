#![allow(dead_code, unused_variables)]

use tokio::{net::TcpStream, select};

#[tokio::main]
async fn main() {
    let mut accept = tokio::net::TcpListener::bind("0.0.0.0:8080");
    let mut connections = futures::stream::FuturesUnordered::new();

    // only 1 future
    loop {
        select! {
            // &mut due to loop
            stream <- (&mut accept).await => {
                connections.push(handle_connection(stream));
            }
            _ <- (&mut connections).await => {}
        }
    }
}

async fn handle_connection(_: TcpStream) {
    todo!()
}
