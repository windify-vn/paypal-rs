use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchAction {
    Add,
    Remove,
    Replace,  
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PatchOperation {
    pub op: PatchAction,
    pub path: String,
    #[builder(default, setter(strip_option))]
    pub value: Option<Value>,
    #[builder(default, setter(strip_option))]
    pub from: Option<String>,
}

impl PatchOperation {
    pub fn replace(path: impl Into<String>, value: Value) -> Self {
        Self {
            op: PatchAction::Replace,
            path: path.into(),
            value: Some(value),
            from: None,
        }
    }

    pub fn add(path: impl Into<String>, value: Value) -> Self {
        Self {
            op: PatchAction::Add,
            path: path.into(),
            value: Some(value),
            from: None,
        }
    }

    pub fn remove(path: impl Into<String>) -> Self {
        Self {
            op: PatchAction::Remove,
            path: path.into(),
            value: None,
            from: None,
        }
    }
}
