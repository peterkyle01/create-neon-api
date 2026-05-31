use std::sync::Arc;

use std::future::Future;
use anyhow::Result;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::Config;
use crate::response::AppError;
use utility_types::Omit;

#[derive(Debug, Clone, Serialize, Deserialize, Omit)]
#[omit(arg(ident=SignInRequest, fields(name), derive(Debug, Clone, Serialize, Deserialize)))]
pub struct SignUpRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub access_token: String,
    pub expires_at: Option<u64>,
}

#[derive(Debug)]
pub struct NeonClient {
    http: Client,
    auth_url: String,
    data_api_url: String,
    jwt_token: Option<String>,
}

impl NeonClient {
    pub fn new(config: &Config) -> Self {
        Self {
            http: Client::new(),
            auth_url: config.auth_url.clone(),
            data_api_url: config.data_api_url.clone(),
            jwt_token: None,
        }
    }

    pub fn with_token(config: &Config, token: String) -> Self {
        Self {
            http: Client::new(),
            auth_url: config.auth_url.clone(),
            data_api_url: config.data_api_url.clone(),
            jwt_token: Some(token),
        }
    }

    #[allow(dead_code)]
    pub fn token(&self) -> Option<&str> {
        self.jwt_token.as_deref()
    }

    pub async fn sign_up(&mut self, email: String, name: String, password: String) -> Result<String, anyhow::Error> {
        let origin = origin_from_url(&self.auth_url);
        let response = self.http
            .post(format!("{}/sign-up/email", self.auth_url))
            .header("Origin", origin)
            .json(&SignUpRequest { email, name, password })
            .send()
            .await?;
        let jwt = extract_jwt_from_response(&response);
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            let msg = serde_json::from_str::<serde_json::Value>(&text).ok()
                .and_then(|v| v.get("message").and_then(|m| m.as_str().map(|s| s.to_string())))
                .unwrap_or_else(|| "request failed".to_string());
            return Err(anyhow::anyhow!("sign_up: {} - {}", status.as_u16(), msg));
        }
        self.jwt_token = jwt;
        self.get_session().await?;
        Ok(self.jwt_token.clone().unwrap_or_default())
    }

    pub async fn sign_in(&mut self, email: String, password: String) -> Result<String, anyhow::Error> {
        let origin = origin_from_url(&self.auth_url);
        let response = self.http
            .post(format!("{}/sign-in/email", self.auth_url))
            .header("Origin", origin)
            .json(&SignInRequest { email, password })
            .send()
            .await?;
        let jwt = extract_jwt_from_response(&response);
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            let msg = serde_json::from_str::<serde_json::Value>(&text).ok()
                .and_then(|v| v.get("message").and_then(|m| m.as_str().map(|s| s.to_string())))
                .unwrap_or_else(|| "request failed".to_string());
            return Err(anyhow::anyhow!("sign_in: {} - {}", status.as_u16(), msg));
        }
        self.jwt_token = jwt;
        self.get_session().await?;
        Ok(self.jwt_token.clone().unwrap_or_default())
    }

    pub async fn get_session(&mut self) -> Result<Option<Session>, reqwest::Error> {
        let token = match &self.jwt_token { Some(t) => t.clone(), None => return Ok(None) };
        let response = self.http
            .get(format!("{}/get-session", self.auth_url))
            .header("Cookie", format!("__Secure-neon-auth.session_token={}", token))
            .send()
            .await?;
        let jwt = response.headers().get("set-auth-jwt").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
        let status = response.status();
        let text = response.text().await.map_err(reqwest::Error::from)?;
        if !status.is_success() { tracing::warn!("get_session status={} body={:?}", status, text); }
        if let Some(jwt) = jwt { self.jwt_token = Some(jwt); }
        let session = serde_json::from_str::<serde_json::Value>(&text).ok()
            .and_then(|v| v.get("session").cloned())
            .and_then(|s| serde_json::from_value::<Session>(s).ok());
        Ok(session)
    }

    pub async fn sign_out(&mut self) -> Result<(), reqwest::Error> {
        let token = match &self.jwt_token { Some(t) => t.clone(), None => return Ok(()) };
        self.http
            .post(format!("{}/sign-out", self.auth_url))
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        self.jwt_token = None;
        Ok(())
    }

    pub async fn create<T: serde::de::DeserializeOwned>(&self, resource: &str, body: impl Serialize) -> Result<Vec<T>, anyhow::Error> {
        Ok(self.http
            .post(format!("{}/{}", self.data_api_url, resource))
            .header("Authorization", format!("Bearer {}", self.bearer_token()?))
            .header("Prefer", "return=representation")
            .json(&body).send().await?.json().await?)
    }

    pub async fn get_all<T: serde::de::DeserializeOwned>(&self, resource: &str) -> Result<Vec<T>, anyhow::Error> {
        Ok(self.http
            .get(format!("{}/{}", self.data_api_url, resource))
            .header("Authorization", format!("Bearer {}", self.bearer_token()?))
            .send().await?.json().await?)
    }

    pub async fn get_one<T: serde::de::DeserializeOwned>(&self, resource: &str, id: i32) -> Result<Option<T>, anyhow::Error> {
        let mut records: Vec<T> = self.http
            .get(format!("{}/{}?id=eq.{}", self.data_api_url, resource, id))
            .header("Authorization", format!("Bearer {}", self.bearer_token()?))
            .send().await?.json().await?;
        Ok(records.pop())
    }

    pub async fn update<T: serde::de::DeserializeOwned>(&self, resource: &str, id: i32, body: impl Serialize) -> Result<Vec<T>, anyhow::Error> {
        Ok(self.http
            .patch(format!("{}/{}?id=eq.{}", self.data_api_url, resource, id))
            .header("Authorization", format!("Bearer {}", self.bearer_token()?))
            .header("Prefer", "return=representation")
            .json(&body).send().await?.json().await?)
    }

    pub async fn delete(&self, resource: &str, id: i32) -> Result<bool, anyhow::Error> {
        let response = self.http
            .delete(format!("{}/{}?id=eq.{}", self.data_api_url, resource, id))
            .header("Authorization", format!("Bearer {}", self.bearer_token()?))
            .header("Prefer", "return=representation")
            .send().await?;
        let text = response.text().await?;
        let rows: Vec<serde_json::Value> = serde_json::from_str(&text).unwrap_or_default();
        Ok(!rows.is_empty())
    }

    fn bearer_token(&self) -> Result<&str, anyhow::Error> {
        self.jwt_token.as_deref().ok_or_else(|| anyhow::anyhow!("not authenticated"))
    }
}

fn origin_from_url(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        format!("{}://{}", parsed.scheme(), parsed.authority())
    } else {
        String::new()
    }
}

fn extract_jwt_from_response(response: &reqwest::Response) -> Option<String> {
    let cookie = response.headers().get("Set-Cookie")?.to_str().ok()?;
    let value = cookie.split(';').next()?.strip_prefix("__Secure-neon-auth.session_token=")?;
    let decoded = urlencoding::decode(value).ok()?;
    Some(decoded.into_owned())
}

impl<S> FromRequestParts<S> for NeonClient
where
    S: Send + Sync,
    Arc<Config>: FromRef<S>,
{
    type Rejection = AppError;

    fn from_request_parts(parts: &mut Parts, state: &S) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let config = Arc::from_ref(state);
        let result = parts.headers.get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Unauthorized("missing or invalid Authorization header".into()))
            .map(|token| NeonClient::with_token(&config, token));
        async move { result }
    }
}
