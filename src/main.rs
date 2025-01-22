use std::{collections::HashMap, env::var};

use afl::run_model;
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<String> {
    println!("Running tipper!");

    let year = 2025;
    let user_email =
        var("AFL_USER_EMAIL").expect("AFL_USER_EMAIL environment variable not set, aborting.");
    let mut offsets: HashMap<String, f64> = HashMap::new();
    offsets.insert("Carlton".to_string(), 19.406364117565367);
    offsets.insert("Essendon".to_string(), 0.7235830990778958);
    offsets.insert("Brisbane Lions".to_string(), 16.520066062563405);
    offsets.insert("St Kilda".to_string(), 22.88624104922586);
    offsets.insert("North Melbourne".to_string(), 2.082236049681039);
    offsets.insert("Hawthorn".to_string(), 25.283426051414427);
    offsets.insert("Greater Western Sydney".to_string(), 22.86020734767821);
    offsets.insert("West Coast".to_string(), 19.999500722606893);
    offsets.insert("Richmond".to_string(), 6.438465748870201);
    offsets.insert("Collingwood".to_string(), 16.26043752475597);
    offsets.insert("Gold Coast".to_string(), 23.963473921487573);
    offsets.insert("Western Bulldogs".to_string(), 18.90258573987233);
    offsets.insert("Adelaide".to_string(), 18.99971873276906);
    offsets.insert("Port Adelaide".to_string(), 28.76221095789279);
    offsets.insert("Sydney".to_string(), 27.068780666497737);
    offsets.insert("Melbourne".to_string(), 0.7707017837905883);
    offsets.insert("Fremantle".to_string(), 17.990320433505854);
    offsets.insert("Geelong".to_string(), 10.699510245724253);

    let (model, margin_model, perf, tips) = run_model(year, None, Some(offsets), user_email).await;
    println!("Model finished");

    let mut model_lines: String = String::new();

    for tip in tips {
        model_lines.push_str(&format!("<h3>{tip}</h3>"));
    }

    for line in format!("{model}").split('\n') {
        model_lines.push_str(&format!("<p>{line}</p>"));
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
