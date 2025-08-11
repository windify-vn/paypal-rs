use crate::endpoints::orders::request::{
    AuthorizeOrderRequest, ConfirmOrderRequest, CreateOrderRequest, ShowOrderDetailRequest,
    UpdateOrderRequest, CaptureOrderRequest
};
use crate::endpoints::orders::response::{
    AuthorizeOrderResponse, ConfirmOrderResponse, CreateOrderResponse, ShowOrderDetailResponse,
    UpdateOrderResponse, CaptureOrderResponse
};
use crate::framework::endpoint::{EndpointSpec, RequestBody};
use http::Method;
use std::borrow::Cow;

impl EndpointSpec for CreateOrderRequest {
    type ResponseType = CreateOrderResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        "v2/checkout/orders".into()
    }

    fn body(&self) -> Option<RequestBody> {
        match serde_json::to_string(self) {
            Ok(json) => Some(RequestBody::Json(json)),
            Err(_) => None,
        }
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed("application/json"))
    }
}

impl EndpointSpec for ShowOrderDetailRequest {
    type ResponseType = ShowOrderDetailResponse;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}", self.order_id)
    }

    fn body(&self) -> Option<RequestBody> {
        None
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        None
    }
}

impl EndpointSpec for UpdateOrderRequest {
    type ResponseType = UpdateOrderResponse;

    fn method(&self) -> Method {
        Method::PATCH
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}", self.order_id)
    }

    fn body(&self) -> Option<RequestBody> {
        match serde_json::to_string(&self.patches) {
            Ok(json) => Some(RequestBody::Json(json)),
            Err(_) => None,
        }
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed("application/json"))
    }
}

impl EndpointSpec for ConfirmOrderRequest {
    type ResponseType = ConfirmOrderResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "v2/checkout/orders/{}/confirm-payment-source",
            self.order_id
        )
    }

    fn body(&self) -> Option<RequestBody> {
        match serde_json::to_string(self) {
            Ok(json) => Some(RequestBody::Json(json)),
            Err(_) => None,
        }
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed("application/json"))
    }
}

impl EndpointSpec for AuthorizeOrderRequest {
    type ResponseType = AuthorizeOrderResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}/authorize", self.order_id)
    }

    fn body(&self) -> Option<RequestBody> {
        match serde_json::to_string(&self) {
            Ok(json) => Some(RequestBody::Json(json)),
            Err(_) => None,
        }
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed("application/json"))
    }
}

impl EndpointSpec for CaptureOrderRequest {
    type ResponseType = CaptureOrderResponse;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}/capture", self.order_id)
    }

    fn body(&self) -> Option<RequestBody> {
        match serde_json::to_string(self) {
            Ok(json) => Some(RequestBody::Json(json)),
            Err(_) => None,
        }
    }

    fn content_type(&self) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed("application/json"))
    }
}

