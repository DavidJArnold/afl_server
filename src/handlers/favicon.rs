use axum::{response::IntoResponse, response::Response};
use std::path::PathBuf;
// use tower_http::services::ServeDir;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn favicon_handler() -> Response {
    println!("Getting favicon");
    let path = PathBuf::from("./static/favicon.ico");
    match File::open(&path).await {
        Ok(mut file) => {
            let mut contents = vec![];
            if file.read_to_end(&mut contents).await.is_ok() {
                Response::builder()
                    .header("Content-Type", "image/x-icon")
                    .body(contents.into_response().into_body())
                    .unwrap()
            } else {
                Response::builder()
                    .status(500)
                    .body("Failed to read favicon".into_response().into_body())
                    .unwrap()
            }
        }
        Err(_) => Response::builder()
            .status(404)
            .body("Favicon not found".into_response().into_body())
            .unwrap(),
    }
}
