use reqwest;
use tokio;
use axum::{routing::get, Router, http::StatusCode};

async fn handler() -> Result<String, (StatusCode, String)> {
    let url = "https://wttr.in/Cape+Town?format=1";
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(body) => Ok(body),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to read response body".to_string())),
        },
        Err(_) => Err((StatusCode::BAD_GATEWAY, "Failed to fetch URL".to_string())),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}