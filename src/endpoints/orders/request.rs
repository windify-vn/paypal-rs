use typed_builder::TypedBuilder;
use crate::endpoints::orders::schema::tracking::{JsonPatchOperation, TrackingRequest, CallbackPayload};

#[derive(Debug, Clone, TypedBuilder)]
pub struct AddOrderTracking {
    #[builder(setter(into))]
    pub order_id: String,

    #[builder(setter(into))]
    pub tracking_request: TrackingRequest,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct UpdateOrCancelOrderTracking {
    #[builder(setter(into))]
    pub order_id: String,

    #[builder(setter(into))]
    pub tracker_id: String,

    #[builder(setter(into))]
    pub operations: Vec<JsonPatchOperation>,
}

#[derive(Debug, Clone)]
pub struct ReceiveOrderUpdateCallback {
    pub payload: CallbackPayload,
}

