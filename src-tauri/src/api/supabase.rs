use reqwest::Client;
use serde_json::Value;
use crate::errors::AppError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthSession {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthUser,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: Option<String>,
}

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

    fn auth_url(&self, path: &str) -> String {
        format!("{}/auth/v1/{}", self.url, path)
    }

    fn rest_url(&self, path: &str) -> String {
        format!("{}/rest/v1/{}", self.url, path)
    }

    pub async fn sign_up(&self, email: &str, password: &str) -> Result<AuthSession, AppError> {
        let body = serde_json::json!({ "email": email, "password": password });
        let resp = self
            .client
            .post(self.auth_url("signup"))
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: Value = resp.json().await?;

        if status.is_success() {
            Ok(AuthSession {
                access_token: data["access_token"].as_str().unwrap_or("").to_string(),
                refresh_token: data["refresh_token"].as_str().unwrap_or("").to_string(),
                user: AuthUser {
                    id: data["user"]["id"].as_str().unwrap_or("").to_string(),
                    email: data["user"]["email"].as_str().map(|s| s.to_string()),
                },
            })
        } else {
            Err(AppError::Auth(
                data["msg"].as_str().unwrap_or("Sign up failed").to_string(),
            ))
        }
    }

    pub async fn sign_in(&self, email: &str, password: &str) -> Result<AuthSession, AppError> {
        let body = serde_json::json!({ "email": email, "password": password });
        let resp = self
            .client
            .post(self.auth_url("token?grant_type=password"))
            .header("apikey", &self.anon_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: Value = resp.json().await?;

        if status.is_success() {
            Ok(AuthSession {
                access_token: data["access_token"].as_str().unwrap_or("").to_string(),
                refresh_token: data["refresh_token"].as_str().unwrap_or("").to_string(),
                user: AuthUser {
                    id: data["user"]["id"].as_str().unwrap_or("").to_string(),
                    email: data["user"]["email"].as_str().map(|s| s.to_string()),
                },
            })
        } else {
            Err(AppError::Auth(
                data["error_description"]
                    .as_str()
                    .unwrap_or("Sign in failed")
                    .to_string(),
            ))
        }
    }

    pub async fn sign_out(&self, access_token: &str) -> Result<(), AppError> {
        self.client
            .post(self.auth_url("logout"))
            .header("apikey", &self.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_user(&self, access_token: &str) -> Result<AuthUser, AppError> {
        let resp = self
            .client
            .get(self.auth_url("user"))
            .header("apikey", &self.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let data: Value = resp.json().await?;
        Ok(AuthUser {
            id: data["id"].as_str().unwrap_or("").to_string(),
            email: data["email"].as_str().map(|s| s.to_string()),
        })
    }

    pub async fn health_check(&self) -> Result<bool, AppError> {
        let resp = self
            .client
            .get(self.rest_url(""))
            .header("apikey", &self.anon_key)
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    pub async fn create_project(
        &self,
        access_token: &str,
        name: &str,
        local_path: &str,
    ) -> Result<Value, AppError> {
        let body = serde_json::json!({
            "name": name,
            "local_path": local_path,
        });

        let resp = self
            .client
            .post(self.rest_url("projects"))
            .header("apikey", &self.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=representation")
            .json(&body)
            .send()
            .await?;

        let data: Value = resp.json().await?;
        Ok(data)
    }

    pub async fn list_projects(&self, access_token: &str) -> Result<Value, AppError> {
        let resp = self
            .client
            .get(&format!(
                "{}?select=*&order=updated_at.desc",
                self.rest_url("projects")
            ))
            .header("apikey", &self.anon_key)
            .header("Authorization", &format!("Bearer {}", access_token))
            .send()
            .await?;

        let data: Value = resp.json().await?;
        Ok(data)
    }
}
