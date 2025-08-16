use crate::endpoints::orders::schema::amount::Amount;
use crate::endpoints::orders::schema::payee::Payee;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum SellerProtectionStatus {
    /// Your PayPal balance remains intact if the customer claims that they did not receive an
    /// item or the account holder claims that they did not authorize the payment.
    Eligible,
    /// Your PayPal balance remains intact if the customer claims that they did not receive an item.
    PartiallyEligible,
    /// This transaction is not eligible for seller protection.
    NotEligible,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SellerProtection {
    /// Indicates whether the transaction is eligible for seller protection.
    /// For information, see PayPal Seller Protection for Merchants.
    pub status: Option<SellerProtectionStatus>,

    /// An array of conditions that are covered for the transaction.
    #[serde(default)]
    pub dispute_categories: Vec<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// The target currency amount. Equivalent to one unit of the source currency.
    /// Formatted as integer or decimal value with one to 15 digits to the right of the decimal point.
    pub value: Option<String>,

    /// The source currency from which to convert an amount.
    pub source_currency: Option<String>,

    /// The target currency to which to convert an amount.
    pub target_currency: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlatformFees {
    /// The fee for this transaction.
    pub amount: Amount,

    /// The recipient of the fee for this transaction. If you omit this value, the default is the API caller.
    pub payee: Option<Payee>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SellerReceivableBreakdown {
    /// An array of platform or partner fees, commissions,
    /// or brokerage fees that associated with the captured payment.
    pub platform_fee: Option<PlatformFees>,

    /// The amount for this captured payment in the currency of the transaction.
    pub gross_amount: Amount,

    /// The applicable fee for this captured payment in the currency of the transaction.
    pub paypal_fee: Option<Amount>,

    /// The applicable fee for this captured payment in the receivable currency.
    /// Returned only in cases the fee is charged in the receivable currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Amount>,

    /// The net amount that the payee receives for this captured payment in their PayPal account.
    /// The net amount is computed as gross_amount minus the paypal_fee minus the platform_fees.
    pub net_amount: Option<Amount>,

    /// The net amount that is credited to the payee's PayPal account.
    /// Returned only when the currency of the captured payment is different from the currency of
    /// the PayPal account where the payee wants to credit the funds.
    /// The amount is computed as net_amount times exchange_rate.
    pub receivable_amount: Option<Amount>,

    /// The exchange rate that determines the amount that is credited to the payee's PayPal account.
    /// Returned when the currency of the captured payment is different from the currency of
    /// the PayPal account where the payee wants to credit the funds.
    pub exchange_rate: Option<ExchangeRate>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetAmountBreakdown {
    /// The net amount debited from the merchant's PayPal account.
    pub payable_amount: Option<Amount>,

    /// The converted payable amount.
    pub converted_amount: Option<Amount>,

    /// The exchange rate that determines the amount that was debited from the merchant's PayPal account.
    pub exchange_rate: Option<ExchangeRate>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SellerPayableBreakdown {
    /// An array of platform or partner fees, commissions, or brokerage fees for the refund.
    pub platform_fee: Option<PlatformFees>,

    /// An array of breakdown values for the net amount.
    /// Returned when the currency of the refund is different from the currency of
    /// the PayPal account where the payee holds their funds.
    #[serde(default)]
    pub net_amount_breakdown: Vec<NetAmountBreakdown>,

    /// The amount that the payee refunded to the payer.
    pub gross_amount: Amount,

    /// The PayPal fee that was refunded to the payer in the currency of the transaction.
    /// This fee might not match the PayPal fee that the payee paid when the payment was captured.
    pub paypal_fee: Option<Amount>,

    /// The PayPal fee that was refunded to the payer in the receivable currency.
    /// Returned only in cases when the receivable currency is different from transaction currency. Example 'CNY'.
    pub paypal_fee_in_receivable_currency: Option<Amount>,

    /// The net amount that the payee's account is debited in the transaction currency.
    /// The net amount is calculated as gross_amount minus paypal_fee minus platform_fees.
    pub net_amount: Option<Amount>,

    /// The net amount that the payee's account is debited in the receivable currency.
    /// Returned only in cases when the receivable currency is different from transaction currency. Example 'CNY'.
    pub net_amount_in_receivable_currency: Option<Amount>,

    /// The total amount refunded from the original capture to date. For example, if
    /// a payer makes a $100 purchase and was refunded $20 a week ago and was refunded $30 in this refund,
    /// the gross_amount is $30 for this refund and the total_refunded_amount is $50.
    pub total_refunded_amount: Option<Amount>,
}
