use warp::Filter;

#[tokio::main]
async fn main() {
    // Create a simple warp filter that responds with "Hello, World!"
    let hello = warp::path!("hello")
        .map(|| warp::reply::html("Hello world"));

    // Start the server on the specified address
    warp::serve(hello)
        .run(([0, 0, 0, 0], 3033)) // binds to localhost:3030
        .await;
}
