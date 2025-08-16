use crate::endpoints::orders::schema::carrier::ShipmentCarrier;
use crate::endpoints::orders::schema::context::ConfirmOrderApplicationContext;
use crate::endpoints::orders::schema::shipment::ShipmentItem;
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
    /// The ID of the order for which to show details.
    #[builder(setter(into))]
    pub order_id: String,

    /// A comma-separated list of fields that should be returned for the order.
    /// Valid filter field is payment_source.
    #[builder(default, setter(strip_option, into))]
    pub fields: Option<HashMap<String, String>>,
}

#[derive(Debug, TypedBuilder)]
pub struct UpdateOrderRequest {
    /// The ID of the order to update.
    #[builder(setter(into))]
    pub order_id: String,

    pub items: Vec<PatchOperatorItem>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct PatchOperatorItem {
    /// The operation.
    #[serde(rename = "op")]
    #[builder(setter(into))]
    pub operator: PatchOperator,

    /// The JSON Pointer to the target document location at which to complete the operation.
    #[builder(setter(into))]
    pub path: String,

    /// The value to apply. The remove, copy, and move operations do not require a value.
    /// Since JSON Patch allows any type for value, the type property is not specified.
    #[builder(default, setter(strip_option, into))]
    pub value: Option<serde_json::Value>,

    /// The JSON Pointer to the target document location from which to move the value.
    /// Required for the move operation.
    #[builder(default, setter(strip_option, into))]
    pub from: Option<String>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct ConfirmOrderRequest {
    /// The ID of the order for which the payer confirms their intent to pay.
    #[builder(setter(into))]
    #[serde(skip)]
    pub order_id: String,

    /// Customizes the payer confirmation experience.
    #[builder(default, setter(strip_option, into))]
    pub application_context: Option<ConfirmOrderApplicationContext>,

    /// The payment source definition.
    #[builder(setter(into))]
    pub payment_source: PaymentSource,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct AuthorizeOrderRequest {
    /// The ID of the order for which to authorize.
    #[builder(setter(into))]
    #[serde(skip)]
    pub order_id: String,

    /// The source of payment for the order, which can be a token or a card.
    /// Use this object only if you have not redirected the user after order creation to approve the payment.
    /// In such cases, the user-selected payment method in the PayPal flow is implicitly used.
    #[builder(setter(into))]
    pub payment_source: Option<PaymentSource>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct CaptureOrderRequest {
    /// The ID of the order for which to capture a payment.
    #[builder(setter(into))]
    #[serde(skip)]
    pub order_id: String,

    /// The ID of the order for which to capture a payment.
    #[builder(setter(into))]
    pub payment_source: Option<PaymentSource>,
}

#[derive(Debug, Serialize, TypedBuilder)]
pub struct PushOrderTrackingRequest {
    /// The ID of the order that the tracking information is associated with.
    #[builder(setter(into))]
    #[serde(skip)]
    pub order_id: String,

    /// The tracking number for the shipment.
    /// This property supports Unicode.
    #[builder(setter(into))]
    pub tracking_number: String,

    /// The name of the carrier for the shipment. Provide this value only if the carrier parameter is OTHER.
    /// This property supports Unicode.
    #[builder(default, setter(into))]
    pub carrier_name_other: Option<String>,

    /// The carrier for the shipment.
    /// Some carriers have a global version as well as local subsidiaries.
    /// The subsidiaries are repeated over many countries and might also have an entry in the global list.
    /// Choose the carrier for your country. If the carrier is not available for your country, choose the global
    /// version of the carrier. If your carrier name is not in the list, set carrier to OTHER and set carrier name
    /// in carrier_name_other. For allowed values, see Carriers.
    #[builder(setter(into))]
    pub carrier: ShipmentCarrier,

    /// The PayPal capture ID.
    #[builder(setter(into))]
    pub capture_id: String,

    /// If true, PayPal will send an email notification to the payer of the PayPal transaction.
    /// The email contains the tracking details provided through the Orders tracking API request.
    /// Independent of any value passed for notify_payer, the payer may receive tracking notifications
    /// within the PayPal app, based on the user's notification preferences.
    #[builder(setter(strip_bool))]
    pub notify_payer: bool,

    /// An array of details of items in the shipment.
    #[builder(default, setter(strip_option, into))]
    pub items: Option<Vec<ShipmentItem>>,
}

#[derive(Debug, TypedBuilder)]
pub struct UpdateOrderTrackingRequest {
    /// The ID of the order that the tracking information is associated with.
    #[builder(setter(into))]
    pub order_id: String,

    /// The order tracking ID.
    #[builder(setter(into))]
    pub tracker_id: String,

    pub items: Vec<PatchOperatorItem>,
}
