use crate::endpoints::orders::request::{AuthorizeOrderRequest, ConfirmOrderRequest, CreateOrderRequest, QueryOrderDetails, UpdateOrderRequest};
use crate::endpoints::orders::response::OrderSummary;
use crate::framework::endpoint::{EndpointSpec, RequestBody, serialize_query};
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

impl EndpointSpec for QueryOrderDetails {
    type ResponseType = OrderSummary;

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}", self.order_id)
    }

    fn query(&self) -> Option<String> {
        self.fields.as_ref().and_then(serialize_query)
    }
}

impl EndpointSpec for UpdateOrderRequest {
    type ResponseType = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn path(&self) -> String {
        format!("v2/checkout/orders/{}", self.order_id)
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(
            serde_json::to_string(&self.items).unwrap(),
        ))
    }
}

impl EndpointSpec for ConfirmOrderRequest {
    type ResponseType = OrderSummary;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "v2/checkout/orders/{}/confirm-payment-source",
            self.order_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(serde_json::to_string(self).unwrap()))
    }
}


impl EndpointSpec for AuthorizeOrderRequest {
    type ResponseType = OrderSummary;

    fn method(&self) -> Method {
        Method::POST
    }

    fn path(&self) -> String {
        format!(
            "v2/checkout/orders/{}/authorize",
            self.order_id
        )
    }

    #[inline]
    fn body(&self) -> Option<RequestBody> {
        Some(RequestBody::Json(serde_json::to_string(self).unwrap()))
    }
}

