use std::sync::Arc;

use serde_json::Value;
use tokio::net::TcpListener;

async fn spawn_app() -> String {
    let config = Arc::new(
        {{project_name}}::config::Config::from_env()
            .expect("AUTH_URL and DATA_API_URL must be set in .env"),
    );
    let app = {{project_name}}::routes(config);
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base = format!("http://{}", addr);
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    base
}

async fn post(url: &str, body: &Value) -> (u16, Value) {
    let client = reqwest::Client::new();
    let resp = client.post(url).json(body).send().await.unwrap();
    (resp.status().as_u16(), resp.json().await.unwrap_or(Value::Null))
}

async fn get_token(base: &str) -> String {
    let (status, body) = post(
        &format!("{}/api/v1/auth/sign-in", base),
        &serde_json::json!({"email": "test@example.com", "password": "password"}),
    ).await;
    assert_eq!(status, 200, "sign-in failed: {:?}", body);
    body["data"]["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_health() {
    let base = spawn_app().await;
    let resp = reqwest::get(&format!("{}/health", base)).await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
}

#[tokio::test]
async fn test_unauthorized() {
    let base = spawn_app().await;
    let resp = reqwest::get(&format!("{}/api/v1/notes", base)).await.unwrap();
    assert_eq!(resp.status().as_u16(), 401);
}

#[tokio::test]
async fn test_notes_crud() {
    let base = spawn_app().await;
    let token = get_token(&base).await;

    let client = reqwest::Client::new();

    // Create
    let resp = client.post(&format!("{}/api/v1/notes", base))
        .header("Authorization", format!("Bearer {}", token))
        .json(&serde_json::json!({"title": "Test", "content": "Hello"}))
        .send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 201);
    let note_id = resp.json().await.unwrap_or(Value::Null);

    // List
    let resp = client.get(&format!("{}/api/v1/notes", base))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);

    // Delete
    let resp = client.delete(&format!("{}/api/v1/notes/{}", base, note_id["data"]["id"].as_i64().unwrap()))
        .header("Authorization", format!("Bearer {}", token))
        .send().await.unwrap();
    assert_eq!(resp.status().as_u16(), 200);
}
