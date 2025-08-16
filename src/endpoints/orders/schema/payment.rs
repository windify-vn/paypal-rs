use crate::endpoints::orders::schema::amount::Amount;
use crate::endpoints::orders::schema::authorization::{
    AuthorizationStatus, AuthorizationStatusDetails,
};
use crate::endpoints::orders::schema::instruction::DisbursementMode;
use crate::endpoints::orders::schema::payer::Payer;
use crate::endpoints::orders::schema::refund::{RefundStatus, RefundStatusDetails};
use crate::endpoints::orders::schema::seller::{
    SellerPayableBreakdown, SellerProtection, SellerReceivableBreakdown,
};
use crate::endpoints::orders::schema::source::card::CardPreviousTransactionReference;
use crate::framework::response::HateoasLink;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PurchaseUnitPayment {
    /// An array of authorized payments for a purchase unit.
    /// A purchase unit can have zero or more authorized payments.
    #[serde(default)]
    pub authorizations: Vec<UnitPaymentAuthorization>,

    /// An array of captured payments for a purchase unit.
    /// A purchase unit can have zero or more captured payments.
    #[serde(default)]
    pub captures: Vec<UnitPaymentCapture>,

    /// An array of refunds for a purchase unit.
    /// A purchase unit can have zero or more refunds.
    #[serde(default)]
    pub refunds: Vec<UnitPaymentRefund>,
}
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProcessorResponse {
    /// The address verification code for Visa, Discover, Mastercard, or American Express transactions.
    pub avs_code: Option<String>,

    /// The card verification value code for for Visa, Discover, Mastercard, or American Express.
    pub cvv_code: Option<String>,

    /// Processor response code for the non-PayPal payment processor errors.
    pub response_code: Option<String>,

    /// The declined payment transactions might have payment advice codes.
    /// The card networks, like Visa and Mastercard, return payment advice codes.
    pub payment_advice_code: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnitPaymentAuthorization {
    /// The status for the authorized payment.
    pub status: Option<AuthorizationStatus>,

    /// The details of the authorized order pending status.
    pub status_details: Option<AuthorizationStatusDetails>,

    /// The PayPal-generated ID for the authorized payment.
    pub id: Option<String>,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with
    /// PayPal transactions. Appears in transaction and settlement reports.
    pub custom_id: Option<String>,

    /// An array of related HATEOAS links.
    #[serde(default)]
    pub links: Vec<HateoasLink>,

    /// The amount for this authorized payment.
    pub amount: Option<Amount>,

    /// Reference values used by the card network to identify a transaction.
    pub network_transaction_reference: Option<CardPreviousTransactionReference>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,

    /// The date and time when the authorized payment expires, in Internet date and time format.
    pub expiration_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The processor response information for payment requests, such as direct credit card transactions.
    pub processor_response: Option<ProcessorResponse>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnitPaymentCapture {
    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The PayPal-generated ID for the captured payment.
    pub id: String,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The API caller-provided external ID. Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: Option<String>,

    /// Indicates whether you can make additional captures against the authorized payment.
    /// Set to true if you do not intend to capture additional payments against the authorization.
    /// Set to false if you intend to capture additional payments against the authorization.
    #[serde(default)]
    pub final_capture: bool,

    /// An array of related HATEOAS links.
    #[serde(default)]
    pub links: Vec<HateoasLink>,

    /// The amount for this captured payment.
    pub amount: Option<Amount>,

    /// Reference values used by the card network to identify a transaction.
    pub network_transaction_reference: Option<CardPreviousTransactionReference>,

    /// The level of protection offered as defined by PayPal Seller Protection for Merchants.
    pub seller_protection: Option<SellerProtection>,

    /// The detailed breakdown of the capture activity.
    /// This is not available for transactions that are in pending state.
    pub seller_receivable_breakdown: Option<SellerReceivableBreakdown>,

    /// The funds that are held on behalf of the merchant.
    pub disbursement_mode: Option<DisbursementMode>,

    /// The processor response information for payment requests, such as direct credit card transactions.
    pub processor_response: Option<ProcessorResponse>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UnitPaymentRefund {
    /// The status of the refund.
    pub status: Option<RefundStatus>,

    /// The details of the refund status.
    pub status_details: Option<RefundStatusDetails>,

    /// The PayPal-generated ID for the refund.
    pub id: Option<String>,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub invoice_id: Option<String>,

    /// The API caller-provided external ID.
    /// Used to reconcile API caller-initiated transactions with PayPal transactions.
    /// Appears in transaction and settlement reports.
    pub custom_id: Option<String>,

    /// Reference ID issued for the card transaction.
    /// This ID can be used to track the transaction across processors, card brands and issuing banks.
    pub acquirer_reference_number: Option<String>,

    /// The reason for the refund.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    pub note_to_payer: Option<String>,

    /// The breakdown of the refund.
    pub seller_payable_breakdown: Option<SellerPayableBreakdown>,

    /// An array of related HATEOAS links.
    #[serde(default)]
    pub links: Vec<HateoasLink>,

    /// The amount that the payee refunded to the payer.
    pub amount: Option<Amount>,

    /// The details associated with the merchant for this transaction.
    pub payer: Option<Payer>,

    /// The date and time when the transaction occurred, in Internet date and time format.
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,

    /// The date and time when the transaction was last updated, in Internet date and time format.
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
}
