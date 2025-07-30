use crate::framework::response::HateoasLink;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{self, Debug};

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Default)]
pub struct ErrorDetail {
    pub issue: String,
    pub field: Option<String>,
    pub value: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub links: Vec<HateoasLink>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiError {
    pub name: String,
    pub message: String,
    pub debug_id: Option<String>,
    #[serde(default)]
    pub details: Vec<ErrorDetail>,
    #[serde(default)]
    pub links: Vec<HateoasLink>,
}

impl Eq for ApiError {}
impl PartialEq for ApiError {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.message == other.message
            && self.details == other.details
            && self.links == other.links
    }
}

#[derive(Debug)]
pub enum ApiFailure {
    Error(reqwest::StatusCode, ApiError),
    Invalid(reqwest::Error),
    Decoding(serde_json::Error),
}

impl Error for ApiFailure {}

impl PartialEq for ApiFailure {
    fn eq(&self, other: &ApiFailure) -> bool {
        match (self, other) {
            (ApiFailure::Invalid(e1), ApiFailure::Invalid(e2)) => e1.to_string() == e2.to_string(),
            (ApiFailure::Error(status1, e1), ApiFailure::Error(status2, e2)) => {
                status1 == status2 && e1 == e2
            }
            (ApiFailure::Decoding(e1), ApiFailure::Decoding(e2)) => {
                e1.to_string() == e2.to_string()
            }
            _ => false,
        }
    }
}
impl Eq for ApiFailure {}

impl fmt::Display for ApiFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiFailure::Error(status, err) => {
                write!(f, "API Error {status}: {}. {}", err.name, err.message)?;
                if let Some(debug_id) = &err.debug_id {
                    write!(f, " (Debug ID: {debug_id})")?;
                }
                if !err.details.is_empty() {
                    write!(f, "\nDetails:")?;
                    for detail in &err.details {
                        write!(f, "\n  - Issue: {}", detail.issue)?;
                        if let Some(field) = &detail.field {
                            write!(f, ", Field: {field}")?;
                        }
                        if let Some(value) = &detail.value {
                            write!(f, ", Value: {value}")?;
                        }
                        if let Some(location) = &detail.location {
                            write!(f, ", Location: {location}")?;
                        }
                        if let Some(description) = &detail.description {
                            write!(f, ", Description: {description}")?;
                        }
                    }
                }
                Ok(())
            }
            ApiFailure::Invalid(err) => write!(f, "{err}"),
            ApiFailure::Decoding(err) => write!(f, "Decoding Error - {err}"),
        }
    }
}

impl From<reqwest::Error> for ApiFailure {
    fn from(error: reqwest::Error) -> Self {
        ApiFailure::Invalid(error)
    }
}
