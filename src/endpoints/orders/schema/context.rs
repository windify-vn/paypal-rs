use crate::endpoints::orders::schema::source::card::CardPreviousTransactionReference;
use crate::endpoints::orders::schema::source::token::PaymentUsage;
use crate::endpoints::orders::schema::source::{PaymentInitiator, PaymentType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct StoredPaymentSource {
    /// The person or party who initiated or triggered the payment.
    #[builder(setter(into))]
    pub payment_initiator: PaymentInitiator,

    /// Indicates the type of the stored payment_source payment.
    #[builder(setter(into))]
    pub payment_type: PaymentType,

    /// The person or party who initiated or triggered the payment.
    #[builder(default, setter(into))]
    pub usage: PaymentUsage,

    /// Reference values used by the card network to identify a transaction.
    #[builder(default, setter(strip_option, into))]
    pub previous_network_transaction_reference: Option<CardPreviousTransactionReference>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ConfirmOrderApplicationContext {
    /// Label to present to your payer as part of the PayPal hosted web experience.
    #[builder(default, setter(strip_option, into))]
    pub brand_name: Option<String>,

    /// The URL where the customer is redirected after the customer approves the payment.
    #[builder(default, setter(strip_option, into))]
    pub return_url: Option<String>,

    /// The URL where the customer is redirected after the customer cancels the payment.
    #[builder(default, setter(strip_option, into))]
    pub cancel_url: Option<String>,

    /// The BCP 47-formatted locale of pages that the PayPal payment experience shows.
    /// PayPal supports a five-character code. For example, da-DK, he-IL, id-ID, ja-JP, no-NO, pt-BR, ru-RU, sv-SE, th-TH, zh-CN, zh-HK, or zh-TW.
    #[builder(default, setter(strip_option, into))]
    pub locale: Option<String>,

    /// Provides additional details to process a payment using a payment_source
    /// that has been stored or is intended to be stored (also referred to as stored_credential or card-on-file).
    #[builder(default, setter(strip_option, into))]
    pub stored_payment_source: Option<StoredPaymentSource>,
}
