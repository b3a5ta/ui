use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contact {
    pub id: String,
    pub name: String,
    pub email: String,
    pub mobile_primary: String,
    pub job_title: Option<String>,
    pub status: String, // Active, Inactive, Lead
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}
