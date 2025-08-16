use crate::endpoints::orders::request::CreateOrderRequest;
use crate::endpoints::orders::response::OrderSummary;
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;

impl EndpointSpec for CreateOrderRequest {
    type ResponseType = OrderSummary;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "/v2/checkout/orders".into()
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(serde_json::to_string(self).unwrap()))
    }
}
