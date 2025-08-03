use crate::endpoints::orders::schema::source::ShippingPreference;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct IdealSource {
    /// The name of the account holder associated with this payment method.
    #[builder(setter(into))]
    pub name: String,

    /// The two-character ISO 3166-1 country code.
    #[builder(setter(into))]
    pub country_code: String,

    /// The bank identification code (BIC).
    #[builder(default, setter(strip_option, into))]
    pub bic: Option<String>,

    /// Customizes the payer experience during the approval process for the payment.
    #[builder(default, setter(strip_option, into))]
    pub experience_context: Option<IdealExperienceContext>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct IdealExperienceContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    /// The pattern is defined by an external party and supports Unicode.
    #[builder(default, setter(strip_option, into))]
    pub brand_name: Option<String>,

    /// The location from which the shipping address is derived.
    #[builder(setter(into))]
    pub shipping_preference: ShippingPreference,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows.
    /// PayPal supports a five-character code.
    /// For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    #[builder(default, setter(strip_option, into))]
    pub locale: Option<String>,

    /// The URL where the customer is redirected after the customer approves the payment.
    #[builder(default, setter(strip_option, into))]
    pub return_url: Option<String>,

    /// The URL where the customer is redirected after the customer cancels the payment.
    #[builder(default, setter(strip_option, into))]
    pub cancel_url: Option<String>,
}
