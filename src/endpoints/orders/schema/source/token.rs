use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    BillingAgreement,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct TokenSource {
    /// The PayPal-generated ID for the token.
    #[builder(setter(into))]
    pub id: String,

    /// The tokenization method that generated the ID.
    #[serde(rename = "type")]
    #[builder(setter(into))]
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaymentUsage {
    /// Indicates the Initial/First payment with a
    /// payment_source that is intended to be stored upon successful processing of the payment.
    First,
    /// Indicates a payment using a stored payment_source which has been successfully used previously for a payment.
    Subsequent,
    /// Indicates that PayPal will derive the value of FIRST or SUBSEQUENT based on data available to PayPal.
    #[default]
    Derived,
}
