use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Candidate {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: Option<String>,
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub degree: Option<String>,
    pub university: Option<String>,
    pub graduation_year: Option<i32>,
    pub gpa: Option<String>,
    pub experience_years: Option<f32>,
    pub current_company: Option<String>,
    pub current_designation: Option<String>,
    pub skills: Option<Vec<String>>,
    pub notice_period: Option<String>,
    pub current_salary: Option<f64>,
    pub expected_salary: Option<f64>,
    pub preferred_dates: Option<String>,
    pub preferred_time: Option<String>,
    pub work_authorization: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub position_applied: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateCandidateRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub location: Option<String>,
    pub linkedin_url: Option<String>,
    pub portfolio_url: Option<String>,
    pub degree: Option<String>,
    pub university: Option<String>,
    pub graduation_year: Option<i32>,
    pub gpa: Option<String>,
    pub experience_years: Option<f32>,
    pub current_company: Option<String>,
    pub current_designation: Option<String>,
    pub skills: Vec<String>,
    pub notice_period: Option<String>,
    pub current_salary: Option<f64>,
    pub expected_salary: Option<f64>,
    pub preferred_dates: Option<String>,
    pub preferred_time: Option<String>,
    pub work_authorization: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub position_applied: Option<String>,
    pub notes: Option<String>,
    pub import_source: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CandidateStats {
    pub total: i64,
    pub by_status: std::collections::HashMap<String, i64>,
}
