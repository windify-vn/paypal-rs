use crate::endpoints::orders::schema::amount::Amount;
use crate::endpoints::orders::schema::billing_plan::BillingPlan;
use crate::endpoints::orders::schema::upc::UniversalProductCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemCategory {
    DigitalGoods,
    PhysicalGoods,
    Donation,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PurchaseItem {
    /// The item name or title.
    #[builder(default, setter(into))]
    pub name: String,

    /// The item quantity. Must be a whole number.
    #[builder(default, setter(into))]
    pub quantity: String,

    /// The detailed item description.
    #[builder(default, setter(strip_option, into))]
    pub description: Option<String>,

    /// The stock keeping unit (SKU) for the item.
    #[builder(default, setter(strip_option, into))]
    pub sku: Option<String>,

    /// The URL to the item being purchased.
    /// Visible to buyer and used in buyer experiences.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,

    /// The item category type.
    #[builder(default, setter(strip_option, into))]
    pub category: Option<ItemCategory>,

    /// The URL of the item's image.
    /// File type and size restrictions apply.
    /// An image that violates these restrictions will not be honored.
    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,

    ///The item price or rate per unit.
    /// If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required.
    /// Must equal unit_amount * quantity for all items.unit_amount.value can not be a negative number
    pub unit_amount: Amount,

    /// The item tax for each unit.
    /// If tax is specified, purchase_units[].amount.breakdown.tax_total is required.
    /// Must equal tax * quantity for all items.tax.value can not be a negative number.
    #[builder(default, setter(strip_option, into))]
    pub tax: Option<Amount>,

    /// The Universal Product Code of the item.
    #[builder(default, setter(strip_option, into))]
    pub upc: Option<UniversalProductCode>,

    /// Metadata for merchant-managed recurring billing plans.
    /// Valid only during the saved payment method token or billing agreement creation.
    #[builder(default, setter(strip_option, into))]
    pub billing_plan: Option<BillingPlan>,
}
