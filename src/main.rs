use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a REST endpoint that responds with the string "Hello world"
    let hello = warp::path!("hello")
        .map(|| warp::reply::with_status("Hello world", warp::http::StatusCode::OK));

    warp::serve(hello)
        .run(([0, 0, 0, 0], 3033))
        .await;
}
