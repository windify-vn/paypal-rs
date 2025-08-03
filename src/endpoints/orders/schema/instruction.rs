use crate::endpoints::orders::schema::amount::Amount;
use crate::endpoints::orders::schema::payee::Payee;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisbursementMode {
    Instant,
    Delayed,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaymentInstruction {
    /// An array of various fees, commissions, tips, or donations.
    /// This field is only applicable to merchants that been enabled for PayPal Complete Payments Platform for Marketplaces and Platforms capability.
    #[builder(default)]
    pub platform_fees: Vec<PlatformFee>,

    /// This field is only enabled for selected merchants/partners to use and provides the ability to
    /// trigger a specific pricing rate/plan for a payment transaction.
    /// The list of eligible 'payee_pricing_tier_id' would be provided to you by your Account Manager.
    /// Specifying values other than the one provided to you by your account manager would result in an error.
    #[builder(default, setter(strip_option, into))]
    pub payee_pricing_tier_id: Option<String>,

    /// FX identifier generated returned by PayPal to be used for payment processing
    /// in order to honor FX rate (for eligible integrations) to be used when amount is settled/received into the payee account.
    #[builder(default, setter(strip_option, into))]
    pub payee_receivable_fx_rate_id: Option<String>,

    /// The funds that are held payee by the marketplace/platform.
    /// This field is only applicable to merchants that been enabled for PayPal Complete Payments Platform for Marketplaces and Platforms capability.
    #[builder(default, setter(strip_option, into))]
    pub disbursement_mode: Option<DisbursementMode>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PlatformFee {
    /// The fee for this transaction.
    pub amount: Amount,

    /// The recipient of the fee for this transaction.
    /// If you omit this value, the default is the API caller.
    #[builder(default, setter(strip_option, into))]
    pub payee: Option<Payee>,
}
