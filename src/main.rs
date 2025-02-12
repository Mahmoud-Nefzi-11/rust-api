use axum::{routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/hello", get(hello_world));

    // Start the server
    axum::Server::bind(&"0.0.0.0:3033".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
