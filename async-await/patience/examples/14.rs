#![allow(dead_code, unused_variables)]

use std::future::Future;

#[tokio::main]
async fn main() {}

struct Request;
struct Response;

trait Service {
    // async fn call(_: Request) -> Response;
    fn call(&mut self, _: Request) -> impl Future<Output = Response>;
}

struct X;
impl Service for X {
    async fn call(&mut self, _: Request) -> Response {
        // Size is really small
        Response
    }
}

struct Y;
impl Service for Y {
    async fn call(&mut self, _: Request) -> Response {
        // Size depends the size of stack variable
        let z = [0; 1024];
        tokio::time::sleep(100).await;
        drop(z);
        Response
    }
}

fn foo(x: &mut dyn Service) {
    // How large is service? unknown size
    let fut = x.call(Request);
}

// #[async_trait]
// rewrite into Pin<Box<dyn Future<Output = Response>>>
// Rust can reason about size
// the problem: heap allocation and pointer indirection
// work really well in high level, not in the lower stack

// To solve this with associate type, which can communicate size
// trait Service {
//   type CallFuture: Future<Output = Response>;
//   fn call(&mut self, _: Request) -> Self::CallFuture;
// }
