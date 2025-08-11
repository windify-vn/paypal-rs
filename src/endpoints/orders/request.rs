use crate::endpoints::orders::schema::PaymentIntent;
use crate::endpoints::orders::schema::source::PaymentSource;
use crate::endpoints::orders::schema::source::paypal::PaypalExperienceContext;
use crate::endpoints::orders::schema::unit::PurchaseUnit;
use crate::endpoints::orders::schema::patch_operation::PatchOperation;
use serde::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateOrderRequest {
    pub intent: PaymentIntent,
    pub purchase_units: Vec<PurchaseUnit>,
    #[builder(default, setter(strip_option, into))]
    pub payment_source: Option<PaymentSource>,
    #[builder(default, setter(strip_option, into))]
    pub application_context: Option<PaypalExperienceContext>,
}

#[derive(Debug, Serialize)]
pub struct ShowOrderDetailRequest {
    #[serde(skip_serializing)]
    pub order_id: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateOrderRequest {
    #[serde(skip_serializing)]
    pub order_id: String,

    #[serde(flatten)]
    pub patches: Vec<PatchOperation>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
pub struct ConfirmOrderRequest {
    #[serde(skip_serializing)]
    pub order_id: String,
    pub payment_source: PaymentSource,
    #[builder(default, setter(strip_option, into))]
    pub application_context: Option<PaypalExperienceContext>,
}

#[derive(Debug, Serialize)]
pub struct AuthorizeOrderRequest {
    #[serde(skip_serializing)]
    pub order_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<PaymentSource>,
}

#[derive(Debug, Serialize)]
pub struct CaptureOrderRequest {
    #[serde(skip_serializing)]
    pub order_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_source: Option<PaymentSource>,
}
