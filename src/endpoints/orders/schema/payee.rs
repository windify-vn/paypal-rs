use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Payee {
    /// The email address of merchant.
    #[builder(default, setter(strip_option, into))]
    pub email_address: Option<String>,

    /// The encrypted PayPal account ID of the merchant.
    #[builder(default, setter(strip_option, into))]
    pub merchant_id: Option<String>,
}
