use std::env::var;

use afl::run_model;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    println!("Running tipper!");

    let year = 2024;
    let user_email =
        var("AFL_USER_EMAIL").expect("AFL_USER_EMAIL environment variable not set, aborting.");
    let (model, margin_model, perf, tips) = run_model(year, None, &user_email).await;
    println!("Model finished");

    let mut model_lines: String = String::new();
    for line in format!("{model}").split('\n') {
        model_lines.push_str(&format!("<p>{line}</p>"));
    }

    for tip in tips {
        model_lines.push_str(&format!("<h3>{tip}</h3>"));
    }

    model_lines.push_str(&format!(
        "<p>{year} score {} from {} games ({:.2}%), first round margin {}</p>",
        perf.total,
        perf.num_games,
        perf.total as f32 / perf.num_games as f32 * 100.0,
        perf.error_margin,
    ));
    let mean_mae = perf.mae as f64 / perf.num_games as f64;
    model_lines.push_str(&format!(
        "<p>MAE: {} BITS: {} (final k={})</p>",
        mean_mae, perf.bits, margin_model.k
    ));

    Html(model_lines)
}
