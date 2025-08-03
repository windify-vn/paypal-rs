use crate::endpoints::orders::schema::source::ShippingPreference;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BlikSource {
    /// The name of the account holder associated with this payment method.
    #[builder(setter(into))]
    pub name: String,

    /// The two-character ISO 3166-1 country code.
    #[builder(setter(into))]
    pub country_code: String,

    /// The email address of the account holder associated with this payment method.
    #[builder(default, setter(strip_option, into))]
    pub email: Option<String>,

    /// Customizes the payer experience during the approval process for the payment.
    #[builder(default, setter(strip_option, into))]
    pub experience_context: Option<BlikExperienceContext>,

    /// The level_0 integration flow object.
    #[builder(default, setter(strip_option, into))]
    pub level_0: Option<BlikLevel0>,

    /// The one-click integration flow object.
    #[builder(default, setter(strip_option, into))]
    pub one_click: Option<BlikOneClick>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BlikExperienceContext {
    /// The label that overrides the business name in the PayPal account on the PayPal site.
    /// The pattern is defined by an external party and supports Unicode.
    #[builder(default, setter(strip_option, into))]
    pub brand_name: Option<String>,

    /// The location from which the shipping address is derived.
    #[builder(default, setter(into))]
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

    /// The payer's User Agent. For example, Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0).
    #[builder(default, setter(strip_option, into))]
    pub consumer_user_agent: Option<String>,

    /// The IP address of the consumer. It could be either IPv4 or IPv6.
    #[builder(default, setter(strip_option, into))]
    pub consumer_ip: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BlikLevel0 {
    /// The 6-digit code used to authenticate a consumer within BLIK.
    #[builder(setter(into))]
    pub auth_code: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BlikOneClick {
    /// The 6-digit code used to authenticate a consumer within BLIK.
    #[builder(default, setter(strip_option, into))]
    pub auth_code: Option<String>,

    /// The merchant generated, unique reference serving as a primary identifier for accounts connected between Blik and a merchant.
    #[builder(setter(into))]
    pub consumer_reference: String,

    /// A bank defined identifier used as a display name to allow the payer to differentiate between multiple registered bank accounts.
    #[builder(default, setter(strip_option, into))]
    pub alias_label: Option<String>,

    /// A Blik-defined identifier for a specific Blik-enabled bank account that is associated with a given merchant.
    /// Used only in conjunction with a Consumer Reference.
    #[builder(default, setter(strip_option, into))]
    pub alias_key: Option<String>,
}
