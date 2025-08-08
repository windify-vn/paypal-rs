use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct TrackingRequest {
    #[builder(setter(into))]
    pub capture_id: String,

    #[builder(setter(into))]
    pub tracking_number: String,

    #[builder(setter(into))]
    pub carrier: String,

    #[builder(setter(strip_bool))]
    pub notify_payer: bool,

    #[builder(default)]
    pub items: Vec<TrackingItem>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct TrackingItem {
    #[builder(default, setter(strip_option, into))]
    pub sku: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub quantity: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub upc: Option<UpcCode>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpcCode {
    #[builder(setter(into))]
    #[serde(rename = "type")]
    pub upc_type: String,

    #[builder(setter(into))]
    pub code: String,
}


#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct JsonPatchOperation {
    #[builder(setter(into))]
    pub op: String,

    #[builder(setter(into))]
    pub path: String,

    #[builder(default, setter(strip_option))]
    pub value: Option<serde_json::Value>,

    #[builder(default, setter(strip_option, into))]
    pub from: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct OrderUpdateCallback {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub intent: String,

    #[builder(setter(into))]
    pub status: String,

    #[builder(default)]
    pub purchase_units: Vec<serde_json::Value>,

    #[builder(default, setter(strip_option))]
    pub payer: Option<serde_json::Value>,

    #[builder(default, setter(strip_option, into))]
    pub create_time: Option<String>,

    #[builder(default, setter(strip_option, into))]
    pub update_time: Option<String>,

    #[builder(default)]
    pub links: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CallbackPayload {
    #[builder(setter(into))]
    pub id: String,

    #[builder(setter(into))]
    pub event_version: String,

    #[builder(setter(into))]
    pub create_time: String,

    #[builder(setter(into))]
    pub resource_type: String,

    #[builder(setter(into))]
    pub resource_version: String,

    #[builder(setter(into))]
    pub event_type: String,

    #[builder(setter(into))]
    pub summary: String,

    #[builder(setter(into))]
    pub resource: OrderUpdateCallback,

    #[builder(default, setter(strip_option))]
    pub links: Vec<serde_json::Value>,
}
