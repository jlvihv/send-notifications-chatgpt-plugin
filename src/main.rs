use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/:topic", post(ntfy))
        .route("/.well-known/ai-plugin.json", get(ai_plugin))
        .route("/openapi.yaml", get(openapi))
        .route("/legal", get(legal));

    if is_running_on_lambda() {
        // Run app on AWS Lambda
        run_hyper_on_lambda(app).await?;
    } else {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}

async fn root() -> &'static str {
    "Hello, This Is Send Notifications ChatGPT Plugin!"
}

#[derive(serde::Deserialize)]
struct Body {
    pub message: String,
}

async fn ntfy(Path(topic): Path<String>, Json(body): Json<Body>) -> (StatusCode, String) {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("https://ntfy.sh/{topic}"))
        .body(body.message)
        .send()
        .await;
    match resp {
        Ok(resp) => (resp.status(), resp.text().await.unwrap_or_default()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

async fn ai_plugin() -> String {
    include_str!("../ai-plugin.json").to_string()
}

async fn openapi() -> String {
    include_str!("../openapi.yaml").to_string()
}

async fn legal() -> String {
    include_str!("../PRIVACY.md").to_string()
}
