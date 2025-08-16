use crate::endpoints::orders::schema::source::PaymentSource;
use crate::endpoints::orders::schema::source::paypal::PaypalExperienceContext;
use crate::endpoints::orders::schema::unit::PurchaseUnit;
use crate::endpoints::orders::schema::{PatchOperator, PaymentIntent};
use serde::Serialize;
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateOrderRequest {
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: PaymentIntent,

    /// An array of purchase units. Each purchase unit establishes a contract between a payer and the payee.
    /// Each purchase unit represents either a full or partial order that the payer intends to purchase from the payee.
    pub purchase_units: Vec<PurchaseUnit>,

    /// The payment source definition.
    #[builder(default, setter(strip_option, into))]
    pub payment_source: Option<PaymentSource>,

    /// Customize the payer experience during the approval process for the payment with PayPal.
    #[builder(default, setter(strip_option, into))]
    pub application_context: Option<PaypalExperienceContext>,
}

#[derive(Debug, TypedBuilder)]
pub struct QueryOrderDetails {
    #[builder(setter(into))]
    pub order_id: String,

    #[builder(default, setter(strip_option, into))]
    pub fields: Option<HashMap<String, String>>,
}

#[derive(Debug, TypedBuilder)]
pub struct UpdateOrderRequest {
    #[builder(setter(into))]
    pub order_id: String,

    pub items: Vec<PathOrderItem>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct PathOrderItem {
    #[serde(rename = "op")]
    #[builder(setter(into))]
    pub operator: PatchOperator,
    #[builder(setter(into))]
    pub path: String,
    #[builder(default, setter(strip_option, into))]
    pub value: Option<serde_json::Value>,
    #[builder(default, setter(strip_option, into))]
    pub from: Option<String>,
}
