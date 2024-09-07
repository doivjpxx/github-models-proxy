mod handler;

use axum::{routing::post, Router};
use handler::response;
use reqwest::Client;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let client = Client::new();

    let app = Router::new()
        .route("/completions", post(response))
        .with_state(client);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
