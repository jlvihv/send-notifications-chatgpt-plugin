use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Hello, This Is Send Notifications Plugin!"
}

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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct Body {
    pub message: String,
}

async fn ntfy(Path(topic): Path<String>, Json(body): Json<Body>) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("https://ntfy.sh/{topic}"))
        .body(body.message)
        .send()
        .await
        .unwrap();
    resp.text().await.unwrap()
}

async fn ai_plugin() -> String {
    r#"{
    "schema_version": "v1",
    "name_for_human": "Send Notifications",
    "name_for_model": "send_notifications",
    "description_for_human": "Plugin for sending notifications to users via ntfy.sh. You can send notifications to any topic.",
    "description_for_model": "Plugin for sending notifications to users via ntfy.sh. You can send notifications to any topic.",
    "auth": {
        "type": "none"
    },
    "api": {
        "type": "openapi",
        "url": "https://cimh4hhtaljvm24cgbbvkwl2gu0eeqkz.lambda-url.us-west-1.on.aws/openapi.yaml",
        "is_user_authenticated": false
    },
    "logo_url": "https://ntfy.sh/_next/static/media/logo.077f6a13.svg",
    "contact_email": "imvihv@gmail.com",
    "legal_info_url": "https://cimh4hhtaljvm24cgbbvkwl2gu0eeqkz.lambda-url.us-west-1.on.aws/legal"
}
"#.to_string()
}

async fn openapi() -> String {
    r#"openapi: 3.0.1
info:
  title: Ntfy.sh Notifications API
  description: An API that allows sending notifications to users via ntfy.sh.
  version: 'v1'
servers:
  - url: https://cimh4hhtaljvm24cgbbvkwl2gu0eeqkz.lambda-url.us-west-1.on.aws
    description: Production server
paths:
  /{topic}:
    post:
      operationId: sendNotification
      summary: Send a notification to a specific topic
      parameters:
        - in: path
          name: topic
          schema:
            type: string
          required: true
          description: The topic to which the notification should be sent.
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              properties:
                message:
                  type: string
                  description: The message to be sent as a notification.
      responses:
        "200":
          description: OK
"#
    .to_string()
}

async fn legal() -> String {
    r#"# Privacy Policy for Send Notifications ChatGPT Plugin

## Introduction

As an individual developer (jlvihv), I am committed to protecting the privacy of my users. This Privacy Policy applies to my plugin, Send Notifications ChatGPT Plugin, and outlines my practices for handling any personal data that my plugin might access.

## Data Collection and Storage

My plugin, Send Notifications ChatGPT Plugin, does not collect or store any personal data. When you use my plugin, I do not access, collect, store, or share any information about you or your usage of the plugin.

## Changes to This Policy

I may update my Privacy Policy from time to time. Thus, you are advised to review this page periodically for any changes. I will notify you of any changes by posting the new Privacy Policy on this page. These changes are effective immediately after they are posted on this page.

## Contact Me

If you have any questions or suggestions about my Privacy Policy, do not hesitate to contact me at imvihv@gmail.com.
"#.to_string()
}
