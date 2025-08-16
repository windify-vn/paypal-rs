use crate::endpoints::orders::schema::payer::Payer;
use crate::endpoints::orders::schema::source::PaymentSource;
use crate::endpoints::orders::schema::unit::PurchaseUnit;
use crate::endpoints::orders::schema::{OrderStatus, PaymentIntent};
use crate::framework::response::{HateoasLink, JsonResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderSummary {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the order.
    #[serde(default)]
    pub id: String,
    /// An array of purchase units. Each purchase unit establishes a contract between a customer and merchant.
    /// Each purchase unit represents either a full or partial order that the customer intends to purchase from the merchant.
    #[serde(default)]
    pub purchase_units: Vec<PurchaseUnit>,

    /// An array of request-related HATEOAS links.
    /// To complete payer approval, use the approve link to redirect the payer.
    /// The API caller has 6 hours (default setting, this which can be changed by
    /// your account manager to 24/48/72 hours to accommodate your use case)
    /// from the time the order is created, to redirect your payer.
    /// Once redirected, the API caller has 6 hours for the payer to approve the order and either authorize or capture the order
    /// If you are not using the PayPal JavaScript SDK to initiate PayPal Checkout (in context) ensure that you include application_context.return_url is specified or you will
    /// get "We're sorry, Things don't appear to be working at the moment" after the payer approves the payment.
    #[serde(default)]
    pub links: Vec<HateoasLink>,
    /// The payment source used to fund the payment.
    pub payment_source: Option<PaymentSource>,
    /// The intent to either capture payment immediately or authorize a payment for an order after order creation.
    pub intent: Option<PaymentIntent>,
    /// The customer who approves and pays for the order. The customer is also known as the payer.
    pub payer: Option<Payer>,
    /// The order status.
    #[serde(default)]
    pub status: OrderStatus,
}

impl JsonResult for OrderSummary {}
