use dioxus::prelude::*;
use gloo_storage::{LocalStorage, Storage};
use reqwest::{Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Serialize};
use crate::models::{
    auth::{LoginRequest, LoginResponse},
    candidate::{Candidate, CandidateStats},
    contact::Contact,
    opportunity::Opportunity,
    interaction::Interaction,
    common::ApiError,
};

const API_BASE_URL: &str = "https://crm-api.futuremug.com/api"; // Replace with actual URL or env var

#[derive(Clone)]
pub struct ApiService {
    client: Client,
}

impl ApiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn get_token(&self) -> Option<String> {
        LocalStorage::get("auth_token").ok()
    }

    async fn request<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let url = format!("{}{}", API_BASE_URL, path);
        let mut req_builder = self.client.request(method, &url);

        if let Some(token) = self.get_token() {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(body) = body {
            req_builder = req_builder.json(body);
        }

        let response = req_builder.send().await.map_err(|e| ApiError {
            message: e.to_string(),
            status: 500,
        })?;

        let status = response.status();
        if status.is_success() {
            response.json::<T>().await.map_err(|e| ApiError {
                message: e.to_string(),
                status: 500,
            })
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(ApiError {
                message: error_text,
                status: status.as_u16(),
            })
        }
    }

    // Auth
    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, ApiError> {
        let res: LoginResponse = self.request(Method::POST, "/auth/login", Some(&req)).await?;
        let _ = LocalStorage::set("auth_token", &res.token);
        let _ = LocalStorage::set("refresh_token", &res.refresh_token);
        let _ = LocalStorage::set("user_data", &res.user);
        Ok(res)
    }

    // Candidates
    pub async fn get_candidates(&self) -> Result<Vec<Candidate>, ApiError> {
        self.request(Method::GET, "/candidates", None::<&()>).await
    }

    pub async fn get_candidate(&self, id: &str) -> Result<Candidate, ApiError> {
        self.request(Method::GET, &format!("/candidates/{}", id), None::<&()>).await
    }

    // Contacts
    pub async fn get_contacts(&self) -> Result<Vec<Contact>, ApiError> {
        self.request(Method::GET, "/contacts", None::<&()>).await
    }

    // Opportunities
    pub async fn get_opportunities(&self) -> Result<Vec<Opportunity>, ApiError> {
        self.request(Method::GET, "/opportunities", None::<&()>).await
    }

    // Interactions
    pub async fn get_interactions(&self) -> Result<Vec<Interaction>, ApiError> {
        self.request(Method::GET, "/interactions", None::<&()>).await
    }
}
