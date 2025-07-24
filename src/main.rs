use axum::{routing::get, Router};
use chrono::Local;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::afl_handler::handler))
        .route("/favicon.ico", get(handlers::favicon::favicon_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let time = Local::now();
    println!("[{time}] listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

