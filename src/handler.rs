use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::{header::AUTHORIZATION, Client};

fn normalize_response(response: reqwest::Response) -> Response<Body> {
    let mut response_builder = Response::builder().status(response.status());
    *response_builder.headers_mut().unwrap() = response.headers().clone();
    response_builder
        .body(Body::from_stream(response.bytes_stream()))
        .unwrap()
}

pub async fn response(State(client): State<Client>, Json(ask): Json<String>) -> Response {
    let base_url = std::env::var("BASE_URL").unwrap();
    let model = std::env::var("GITHUB_MODEL").unwrap();
    let token = std::env::var("GITHUB_TOKEN").unwrap();
    let sys_prompt = std::env::var("SYSTEM_PROMPT").unwrap();

    let url = format!("{}/chat/completions", base_url);
    let reqwest_response = match client
        .post(url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&serde_json::json!({
            "model": model,
            "stream": true,
            "messages": [
                {
                    "role": "system",
                    "content": sys_prompt
                },
                {
                    "role": "user",
                    "content": ask,
                }
            ]
        }))
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, Body::empty()).into_response();
        }
    };

    normalize_response(reqwest_response)
}
