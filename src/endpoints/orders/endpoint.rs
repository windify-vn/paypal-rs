use crate::framework::endpoint::{EndpointSpec, Method, RequestBody};
use crate::endpoints::orders::{AddOrderTracking, AddTrackingResponse, ReceiveOrderUpdateCallback, UpdateOrCancelOrderTracking};
use crate::endpoints::orders::response::CallbackResponse;

impl EndpointSpec for AddOrderTracking {
    type ResponseType = AddTrackingResponse;
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        format!("/v2/checkout/orders/{}/track", self.order_id)
    }
    fn body(&self) -> Option<RequestBody> {
        let json = serde_json::to_string(&self.tracking_request).unwrap();
        Some(RequestBody::Json(json))
    }
}

impl EndpointSpec for UpdateOrCancelOrderTracking {
    type ResponseType = ();
    fn method(&self) -> Method {
        Method::PATCH
    }
    fn path(&self) -> String {
        format!("/v2/checkout/orders/{}/trackers/{}", self.order_id, self.tracker_id)
    }
    fn body(&self) -> Option<RequestBody> {
        let json = serde_json::to_string(&self.operations).unwrap();
        Some(RequestBody::Json(json))
    }
}

impl EndpointSpec for ReceiveOrderUpdateCallback {
    type ResponseType = CallbackResponse;
    fn method(&self) -> Method {
        Method::POST
    }
    fn path(&self) -> String {
        "/v2/checkout/orders/callback".to_string()
    }
    fn body(&self) -> Option<RequestBody> {
        let json = serde_json::to_string(&self.payload).unwrap();
        Some(RequestBody::Json(json))
    }
}

