use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Opportunity {
    pub id: String,
    pub title: String,
    pub company_name: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub requirements: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
