use axum::{
    routing::{get, post},
    Router,
};
use neptune::App;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("3000"));

    println!("Listening on `0.0.0.0:{port}`");
    axum::serve(
        TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap(),
        Router::new()
            .route("/add_batch", post(App::add_batch))
            .route("/stats", get(App::stats))
            .with_state(App::default()),
    )
    .await
    .unwrap();
}
