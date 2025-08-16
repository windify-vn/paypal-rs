use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum AuthorizationStatus {
    /// The authorized payment is created.
    /// No captured payments have been made for this authorized payment.
    Created,
    /// The authorized payment has one or more captures against it.
    /// The sum of these captured payments is greater than the amount of the original authorized payment.
    Captured,
    /// PayPal cannot authorize funds for this authorized payment.
    Denied,
    /// A captured payment was made for the authorized payment for an
    /// amount that is less than the amount of the original authorized payment.
    PartiallyCaptured,
    /// The authorized payment was voided.
    /// No more captured payments can be made against this authorized payment.
    Voided,
    /// The created authorization is in pending state. For more information, see status.details.
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum AuthorizationStatusReason {
    /// Authorization is pending manual review.
    PendingReview,
    /// Risk Filter set by the payee failed for the transaction.
    DeclinedByRiskFraudFilters,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AuthorizationStatusDetails {
    /// The reason why the authorized status is PENDING.
    pub reason: Option<AuthorizationStatusReason>,
}
