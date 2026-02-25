use crate::error::{BayarcashError, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;

pub(crate) struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new(base_url: &str, token: &str, timeout: Duration) -> Result<Self> {
        let client = Client::builder()
            .timeout(timeout)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", token).parse().unwrap(),
                );
                headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
                headers
            })
            .build()
            .map_err(BayarcashError::Http)?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .post(&url)
            .form(body)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .put(&url)
            .form(body)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}/{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .form(body)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T> {
        let response = self
            .client
            .post(url)
            .multipart(form)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    /// Post form data to an absolute URL (not relative to base_url)
    pub async fn post_absolute<T: DeserializeOwned, B: Serialize>(
        &self,
        url: &str,
        body: &B,
    ) -> Result<T> {
        let response = self
            .client
            .post(url)
            .form(body)
            .send()
            .await
            .map_err(BayarcashError::Http)?;
        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            return response.json::<T>().await.map_err(BayarcashError::Http);
        }

        let body = response.text().await.unwrap_or_default();

        match status.as_u16() {
            422 => {
                let errors = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| {
                        v.get("errors").and_then(|e| {
                            serde_json::from_value::<HashMap<String, Vec<String>>>(e.clone()).ok()
                        })
                    })
                    .unwrap_or_default();

                let message = serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|v| v.get("message").and_then(|m| m.as_str().map(String::from)))
                    .unwrap_or_else(|| "Validation failed".to_string());

                Err(BayarcashError::Validation { message, errors })
            }
            404 => Err(BayarcashError::NotFound),
            400 => {
                let parsed = serde_json::from_str::<serde_json::Value>(&body).ok();
                let message = parsed
                    .as_ref()
                    .and_then(|v| v.get("message").and_then(|m| m.as_str().map(String::from)))
                    .unwrap_or_else(|| "Action failed".to_string());
                Err(BayarcashError::FailedAction {
                    message,
                    details: parsed,
                })
            }
            429 => Err(BayarcashError::RateLimitExceeded { reset_at: None }),
            _ => Err(BayarcashError::Other(format!(
                "HTTP {} - {}",
                status.as_u16(),
                body.chars().take(200).collect::<String>()
            ))),
        }
    }
}
