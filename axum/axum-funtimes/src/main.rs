use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

// tower::Service
// is roughly equivalent to something that implements
//
// async fn a(Request) -> Result<Response, E> for some <Request, E>
// async fn b(Request) -> Result<Response, E> for some <Request, E>
//
// async fn c(r: Request) -> Result<Response, E> for some <Request, E> {
//   (a.layer(b))(req)
// }

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    //   Router::<S <= Arc<AppState>, B = BoxBody>::new()
    let app = Router::new()
        // `GET /` goes to `root`
        // .route("/", get::<H = root, T = (), S <= Arc<AppState>, B = BoxBody>(root))
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*
type H = Box<dyn Service<http::Request, Response = http::Respose>>;

struct PathHandler {
    paths: HashMap<String, H>,
}

impl Service<http::Request> for PathHandler {
    type Response = http::Response;

    async fn call(req: http::Request) -> Self::Response {
        if let Some(handler) = self.paths.get(&req.path) {
            return handler(req).await;
        }

        self.fallback(req).await
    }
}

struct MethodHandler {
    get: Option<H>,
    post: Option<H>,
    fallback: H,
}

impl Service<http::Request> for MethodHandler {
    type Response = http::Response;

    async fn call(req: http::Request) -> Self::Response {
        match req.method {
            Method::GET if self.get.is_some() => return self.get(req).await,
            Method::POST if self.post.is_some() => return self.post(req).await,
            _ => self.fallback(req).await,
        }
    }
}
*/

// basic handler that responds with a static string
// does it return a Fut: Future  -- check
// does Fut::Output: IntoResponse -- check
// does every argument except the last implement FromRequestParts -- check
// does the last argument implement FromRequest -- check
//
// doesn't implement FromRequestParts<S> for any S
// *only* implements FromRequestParts<Arc<AppState>>
//
// S = Arc<AppState>>
//
// impl Handler<T, Arc<AppState>> for root
async fn root() -> &'static str {
    // is somehow transformed into
    //
    //   async fn(http::Request) -> http::Response
    //
    // by axum.
    "Hello, World!"
}

// does it return a Fut: Future  -- check
// does Fut::Output: IntoResponse -- check
// does every argument except the last implement FromRequestParts -- check
// does the last argument implement FromRequest -- check
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
