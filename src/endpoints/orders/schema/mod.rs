use serde::{Deserialize, Serialize};

pub mod address;
pub mod amount;
pub mod billing_plan;
pub mod countries;
pub mod instruction;
pub mod item;
pub mod payee;
pub mod payer;
pub mod shipping;
pub mod source;
pub mod supplementary;
pub mod unit;
pub mod upc;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaymentIntent {
    /// The merchant intends to capture payment immediately after the customer makes a payment.
    Capture,
    /// The merchant intends to authorize a payment and place funds on hold after the customer makes a payment.
    /// Authorized payments are best captured within three days of authorization but are available to capture for up to 29 days.
    /// After the three-day honor period, the original authorized payment expires and you must re-authorize the payment.
    /// You must make a separate request to capture payments on demand. This intent is not supported when you have more than one purchase_unit within your order.
    Authorize,
}

#[derive(Debug, Serialize, Default, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum OrderStatus {
    /// The order was created with the specified context.
    #[default]
    Created,
    /// The order was saved and persisted. The order status continues to be in progress
    /// until a capture is made with final_capture = true for all purchase units within the order.
    Saved,
    /// The customer approved the payment through the PayPal wallet or another form of guest or unbranded payment.
    /// For example, a card, bank account, or so on.
    Approved,
    /// All purchase units in the order are voided.
    Voided,
    /// The intent of the order was completed and a payments resource was created.
    /// Important: Check the payment status in purchase_units[].payments.captures[].status before fulfilling the order.
    /// A completed order can indicate a payment was authorized, an authorized payment was captured, or a payment was declined.
    Completed,
    /// The order requires an action from the payer (e.g. 3DS authentication).
    /// Redirect the payer to the "rel":"payer-action" HATEOAS link returned as part of the response prior
    /// to authorizing or capturing the order. Some payment sources may not return a payer-action HATEOAS link (eg. MB WAY).
    /// For these payment sources the payer-action is managed by the scheme itself (eg. through SMS, email, in-app notification, etc).
    PayerActionRequired,
}
