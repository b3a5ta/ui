use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Interaction {
    pub id: String,
    pub title: String,
    pub interaction_type: String, // Call, Email, Meeting, Note
    pub date: String,
    pub time: Option<String>,
    pub description: Option<String>,
    pub entity_type: Option<String>, // Lead, Opportunity
    pub entity_id: Option<String>,
    pub created_at: Option<String>,
}
