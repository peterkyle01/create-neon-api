use crate::{errors::AppError, models::User};
use reqwest::Client;
use serde_json::Value;
use tracing::debug;

pub struct DataApi {
    client: Client,
    base_url: String,
    anon_key: Option<String>,
}

impl DataApi {
    pub fn new(client: Client, base_url: String, anon_key: Option<String>) -> Self {
        Self { client, base_url, anon_key }
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
    }

    fn auth<'a>(&'a self, token: Option<&'a str>) -> Option<&'a str> {
        token.or(self.anon_key.as_deref())
    }

    // -- low-level HTTP helpers --

    pub async fn get(&self, path: &str, token: Option<&str>) -> Result<Value, AppError> {
        let mut req = self.client.get(&self.url(path));
        if let Some(t) = self.auth(token) { req = req.bearer_auth(t); }
        debug!("GET {}", self.url(path));
        Ok(req.send().await?.json().await?)
    }

    pub async fn post(&self, path: &str, body: &Value, token: Option<&str>) -> Result<Value, AppError> {
        let mut req = self.client.post(&self.url(path))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation");
        if let Some(t) = self.auth(token) { req = req.bearer_auth(t); }
        debug!("POST {} body={}", self.url(path), body);
        Ok(req.json(body).send().await?.json().await?)
    }

    #[allow(dead_code)]
    pub async fn patch(&self, path: &str, body: &Value, token: Option<&str>) -> Result<Value, AppError> {
        let mut req = self.client.patch(&self.url(path))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation");
        if let Some(t) = self.auth(token) { req = req.bearer_auth(t); }
        debug!("PATCH {} body={}", self.url(path), body);
        Ok(req.json(body).send().await?.json().await?)
    }

    #[allow(dead_code)]
    pub async fn delete(&self, path: &str, token: Option<&str>) -> Result<Value, AppError> {
        let mut req = self.client.delete(&self.url(path));
        if let Some(t) = self.auth(token) { req = req.bearer_auth(t); }
        debug!("DELETE {}", self.url(path));
        Ok(req.send().await?.json().await?)
    }

    // -- typed helpers --

    pub async fn find_user_by_email(&self, email: &str, token: Option<&str>) -> Result<Option<User>, AppError> {
        let path = format!("users?email=eq.{}&limit=1", email);
        let body = self.get(&path, token).await?;
        Ok(serde_json::from_value::<Vec<User>>(body).unwrap_or_default().into_iter().next())
    }

    pub async fn find_user_by_id(&self, id: &str, token: Option<&str>) -> Result<Option<User>, AppError> {
        let path = format!("users?id=eq.{}&limit=1", id);
        let body = self.get(&path, token).await?;
        Ok(serde_json::from_value::<Vec<User>>(body).unwrap_or_default().into_iter().next())
    }

    pub async fn insert_user(&self, user: &Value, token: Option<&str>) -> Result<User, AppError> {
        let body = self.post("users", user, token).await?;
        serde_json::from_value::<Vec<User>>(body)
            .map_err(|e| AppError::Internal(format!("failed to parse user: {e}")))?
            .into_iter()
            .next()
            .ok_or_else(|| AppError::Internal("no user returned after insert".into()))
    }
}
