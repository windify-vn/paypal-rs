use crate::endpoints::orders::schema::amount::AmountBreakdown;
use crate::endpoints::orders::schema::instruction::PaymentInstruction;
use crate::endpoints::orders::schema::item::PurchaseItem;
use crate::endpoints::orders::schema::payee::Payee;
use crate::endpoints::orders::schema::payment::PurchaseUnitPayment;
use crate::endpoints::orders::schema::shipping::Shipping;
use crate::endpoints::orders::schema::supplementary::SupplementaryData;
use serde::Deserialize;
use serde_with::serde_derive::Serialize;
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PurchaseUnit {
    /// The API caller-provided external ID for the purchase unit.
    /// Required for multiple purchase units when you must update the order through PATCH.
    /// If you omit this value and the order contains only one purchase unit, PayPal sets this value to default.
    #[builder(default, setter(into))]
    pub reference_id: Option<String>,

    /// The purchase description.
    /// The maximum length of the character is dependent on the type of characters used.
    /// The character length is specified assuming a US ASCII character.
    /// Depending on type of character; (e.g. accented character, Japanese characters) the number of characters
    /// that that can be specified as input might not equal the permissible max length.
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,

    /// The API caller-provided external ID.
    /// Used to reconcile client transactions with PayPal transactions.
    /// Appears in transaction and settlement reports but is not visible to the payer.
    #[builder(default, setter(strip_option, into))]
    pub custom_id: Option<String>,

    /// The API caller-provided external invoice number for this order.
    /// Appears in both the payer's transaction history and the emails that the payer receives.
    #[builder(default, setter(into))]
    pub invoice_id: Option<String>,

    /// The PayPal-generated ID for the purchase unit.
    /// This ID appears in both the payer's transaction history and the emails that the payer receives.
    /// In addition, this ID is available in transaction and settlement reports that merchants and API callers can use to reconcile transactions.
    /// This ID is only available when an order is saved by calling v2/checkout/orders/id/save.
    /// You shouldn't set this field
    #[builder(default)]
    pub id: Option<String>,

    /// The soft descriptor is the dynamic text used to construct the statement descriptor that appears on a payer's card statement.
    /// If an Order is paid using the "PayPal Wallet",
    /// the statement descriptor will appear in following format on the
    /// payer's card statement: PAYPAL_prefix+(space)+merchant_descriptor+(space)+ soft_descriptor
    #[builder(default, setter(strip_option, into))]
    pub soft_descriptor: Option<String>,

    /// An array of items that the customer purchases from the merchant.
    #[builder(default)]
    pub items: Option<Vec<PurchaseItem>>,

    /// The total order amount with an optional breakdown that provides details, such as the total item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
    /// If you specify amount.breakdown, the amount equals item_total plus tax_total plus shipping plus handling plus insurance minus shipping_discount minus discount.
    /// The amount must be a positive number. The amount.value field supports up to 15 digits preceding the decimal.
    /// For a list of supported currencies, decimal precision, and maximum charge amount, see the PayPal REST APIs Currency Codes.
    pub amount: AmountBreakdown,

    /// The merchant who receives payment for this transaction.
    #[builder(default, setter(strip_option, into))]
    pub payee: Option<Payee>,

    /// Any additional payment instructions to be consider during payment processing.
    /// This processing instruction is applicable for Capturing an order or Authorizing an Order.
    #[builder(default, setter(strip_option, into))]
    pub payment_instruction: Option<PaymentInstruction>,

    /// The name and address of the person to whom to ship the items.
    #[builder(default, setter(strip_option, into))]
    pub shipping: Option<Shipping>,

    /// Contains Supplementary Data.
    #[builder(default, setter(strip_option, into))]
    pub supplementary_data: Option<SupplementaryData>,

    /// The comprehensive history of payments for the purchase unit.
    #[builder(default)]
    pub payments: Option<PurchaseUnitPayment>,
}
