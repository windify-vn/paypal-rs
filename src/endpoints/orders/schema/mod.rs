use serde::{Deserialize, Serialize};

mod address;
pub mod amount;
pub mod billing_plan;
pub mod countries;
mod instruction;
pub mod item;
mod payee;
mod shipping;
pub mod source;
mod supplementary;
pub mod unit;
pub mod upc;
pub mod patch_operation;

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
