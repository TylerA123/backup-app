use reqwest::Client;
use serde_json::Value;
use crate::errors::AppError;

pub struct SupabaseClient {
    client: Client,
    url: String,
    anon_key: String,
}

impl SupabaseClient {
    pub fn new(url: String, anon_key: String) -> Self {
        SupabaseClient {
            client: Client::new(),
            url,
            anon_key,
        }
    }

    pub async fn health_check(&self) -> Result<bool, AppError> {
        let response = self
            .client
            .get(&format!("{}/rest/v1/", self.url))
            .header("apikey", &self.anon_key)
            .send()
            .await?;
        Ok(response.status().is_success())
    }

    pub async fn create_project(&self, user_id: &str, name: &str, local_path: &str) -> Result<Value, AppError> {
        let body = serde_json::json!({
            "user_id": user_id,
            "name": name,
            "local_path": local_path,
        });

        let response = self
            .client
            .post(&format!("{}/rest/v1/projects", self.url))
            .header("apikey", &self.anon_key)
            .header("Authorization", "Bearer")
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&body)
            .send()
            .await?;

        let data: Value = response.json().await?;
        Ok(data)
    }

    pub async fn list_projects(&self, access_token: &str) -> Result<Value, AppError> {
        let response = self
            .client
            .get(&format!("{}/rest/v1/projects?select=*&order=updated_at.desc", self.url))
            .header("apikey", &self.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let data: Value = response.json().await?;
        Ok(data)
    }
}
