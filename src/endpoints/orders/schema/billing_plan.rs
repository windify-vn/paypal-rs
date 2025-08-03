use crate::endpoints::orders::schema::amount::Amount;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// The tenure type of the billing cycle identifies
/// if the billing cycle is a trial(free or discounted) or regular billing cycle.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum TenureType {
    Regular,
    Trial,
}

/// The pricing model for the billing cycle.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PricingModel {
    Fixed,
    Variable,
    AutoReload,
}

/// Metadata for merchant-managed recurring billing plans.
/// Valid only during the saved payment method token or billing agreement creation.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BillingPlan {
    /// An array of billing cycles for trial billing and regular billing.
    /// A plan can have at most two trial cycles and only one regular cycle.
    pub billing_cycles: Vec<BillingCycle>,

    /// Name of the recurring plan.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    /// The setup fee for the recurring plan.
    /// Ensure its part of the item amount.
    #[builder(default, setter(strip_option, into))]
    pub setup_fee: Option<Amount>,
}

/// The active pricing scheme for this billing cycle.
/// A free trial billing cycle does not require a pricing scheme.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PricingScheme {
    /// The pricing model for the billing cycle.
    pub pricing_model: PricingModel,

    /// The price the customer will be charged based on the pricing model
    #[builder(default, setter(strip_option))]
    pub price: Option<Amount>,

    /// The threshold amount on which the reload charge would be triggered.
    /// This will be associated with the account-balance where if the account-balance goes below this amount then customer would incur reload charge.
    #[builder(default, setter(strip_option))]
    pub reload_threshold_amount: Option<Amount>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BillingCycle {
    /// The tenure type of the billing cycle identifies if the
    /// billing cycle is a trial(free or discounted) or regular billing cycle.
    pub tenure_type: TenureType,

    /// The number of times this billing cycle gets executed.
    /// Trial billing cycles can only be executed a finite number of times (value between 1 and 999 for total_cycles).
    /// Regular billing cycles can be executed infinite times (value of 0 for total_cycles) or a finite number of times (value between 1 and 999 for total_cycles).
    #[builder(default, setter(strip_option, into))]
    pub total_cycles: Option<u8>,

    /// The order in which this cycle is to run among other billing cycles.
    /// For example, a trial billing cycle has a sequence of 1 while a regular billing cycle has a sequence of 2,
    /// so that trial cycle runs before the regular cycle.
    #[builder(default, setter(strip_option, into))]
    pub sequence: Option<u8>,

    /// The active pricing scheme for this billing cycle.
    /// A free trial billing cycle does not require a pricing scheme.
    #[builder(default, setter(strip_option, into))]
    pub pricing_scheme: Option<PricingScheme>,

    /// The start date for the billing cycle.
    /// This field should be not be provided if the billing cycle starts at the time of checkout.
    /// When this field is not provided, the billing cycle amount will be included in any data validations confirming that the
    /// total provided by the merchant match the sum of individual items due at the time of checkout.
    /// Only one billing cycle (with sequence equal to 1) can have a no start date.
    #[builder(default, setter(strip_option, into))]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
}
