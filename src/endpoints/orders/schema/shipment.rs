use crate::endpoints::orders::schema::upc::UniversalProductCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ShipmentItem {
    /// The item name or title.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    /// The item quantity. Must be a whole number.
    #[builder(default, setter(strip_option, into))]
    pub quantity: Option<String>,

    /// The stock keeping unit (SKU) for the item. This can contain unicode characters.
    #[builder(default, setter(strip_option, into))]
    pub sku: Option<String>,

    /// The URL to the item being purchased. Visible to buyer and used in buyer experiences.
    #[builder(default, setter(strip_option, into))]
    pub url: Option<String>,

    /// The URL of the item's image. File type and size restrictions apply.
    /// An image that violates these restrictions will not be honored.
    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,

    /// The Universal Product Code of the item.
    #[builder(default, setter(strip_option, into))]
    pub upc: Option<UniversalProductCode>,
}
