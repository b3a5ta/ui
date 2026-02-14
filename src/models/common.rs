use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApiError {
    pub message: String,
    pub status: u16,
}
