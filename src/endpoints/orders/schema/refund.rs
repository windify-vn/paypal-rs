use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum RefundStatus {
    /// The refund was cancelled.
    Cancelled,
    /// The refund could not be processed.
    Failed,
    /// The refund is pending. For more information, see status_details.reason.
    Pending,
    /// The funds for this transaction were debited to the customer's account.
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum RefundStatusReason {
    /// The customer's account is funded through an eCheck, which has not yet cleared.
    Echeck,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RefundStatusDetails {
    /// The reason why the authorized status is PENDING.
    pub reason: Option<RefundStatusReason>,
}
