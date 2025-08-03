use crate::endpoints::orders::schema::address::Address;
use crate::endpoints::orders::schema::amount::Amount;
use crate::endpoints::orders::schema::billing_plan::BillingPlan;
use crate::endpoints::orders::schema::upc::UniversalProductCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct SupplementaryData {
    /// Merchants and partners can add Level 2 and 3 data to payments to reduce risk and payment processing costs.
    /// For more information about processing payments, see checkout or multiparty checkout.
    #[builder(default, setter(strip_option, into))]
    pub card: Option<CardData>,

    /// Merchants and partners can add additional customer parameters that
    /// can help with better fraud protection and reduced risk for unbranded card payments.
    #[builder(default, setter(strip_option, into))]
    pub risk: Option<RiskData>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct RiskData {
    /// Profile information of the sender or receiver.
    #[builder(default, setter(strip_option, into))]
    pub customer: Option<CustomerData>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CustomerData {
    /// The consumer's IP address, which can be represented in either IPv4 or IPv6 format.
    #[builder(default, setter(strip_option, into))]
    pub ip_address: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CardData {
    /// The level 2 card processing data collections.
    /// If your merchant account has been configured for Level 2 processing this field will be passed to the processor on your behalf.
    /// Please contact your PayPal Technical Account Manager to define level 2 data for your business.
    #[builder(default, setter(strip_option, into))]
    pub level_2: Option<CardLevel2Data>,

    /// The level 3 card processing data collections,
    /// If your merchant account has been configured for Level 3 processing this field will be passed to the processor on your behalf.
    /// Please contact your PayPal Technical Account Manager to define level 3 data for your business.
    #[builder(default, setter(strip_option, into))]
    pub level_3: Option<CardLevel3Data>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CardLevel2Data {
    /// Use this field to pass a purchase identification value of up to 127 ASCII characters.
    /// The length of this field will be adjusted to meet network specifications (25chars for Visa and Mastercard, 17chars for Amex),
    /// and the original invoice ID will still be displayed in your existing reports.
    #[builder(default, setter(strip_option, into))]
    pub invoice_id: Option<String>,

    /// Use this field to break down the amount of tax included in the total purchase amount.
    /// The value provided here will not add to the total purchase amount.
    /// The value can't be negative, and in most cases, it must be greater than zero in order to qualify for lower interchange rates. Value, by country, is:
    #[builder(default, setter(strip_option, into))]
    pub tax_total: Option<Amount>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CardLevel3Data {
    /// Use this field to specify the postal code of the shipping location.
    #[builder(default, setter(strip_option, into))]
    pub ships_from_postal_code: Option<String>,

    /// Use this field to break down the shipping cost included in the total purchase amount.
    /// The value provided here will not add to the total purchase amount. The value cannot be negative.
    #[builder(default, setter(strip_option, into))]
    pub shipping_amount: Option<Amount>,

    /// Use this field to break down the duty amount included in the total purchase amount.
    /// The value provided here will not add to the total purchase amount. The value cannot be negative.
    #[builder(default, setter(strip_option, into))]
    pub duty_amount: Option<Amount>,

    /// Use this field to break down the discount amount included in the total purchase amount.
    /// The value provided here will not add to the total purchase amount. The value cannot be negative.
    #[builder(default, setter(strip_option, into))]
    pub discount_amount: Option<Amount>,

    /// A list of the items that were purchased with this payment.
    /// If your merchant account has been configured for Level 3 processing this field will be passed to the processor on your behalf.
    #[builder(default)]
    pub line_items: Vec<CardDataItem>,

    /// The address of the person to whom to ship the items.
    /// Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    #[builder(default, setter(strip_option, into))]
    pub shipping_address: Option<Address>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CardDataItem {
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

    /// The URL of the item's image.
    /// File type and size restrictions apply.
    /// An image that violates these restrictions will not be honored.
    #[builder(default, setter(strip_option, into))]
    pub image_url: Option<String>,

    /// The Universal Product Code of the item.
    #[builder(default, setter(strip_option, into))]
    pub upc: Option<UniversalProductCode>,

    /// Metadata for merchant-managed recurring billing plans.
    /// Valid only during the saved payment method token or billing agreement creation.
    #[builder(default, setter(strip_option, into))]
    pub billing_plan: Option<BillingPlan>,

    /// Code used to classify items purchased and track the total amount spent across various categories of products and services.
    /// Different corporate purchasing organizations may use different standards, but the United Nations Standard Products and Services Code (UNSPSC) is frequently used.
    #[builder(default, setter(strip_option, into))]
    pub commodity_code: Option<String>,

    /// Unit of measure is a standard used to express the magnitude of a quantity in international trade.
    /// Most commonly used (but not limited to) examples are: Acre (ACR), Ampere (AMP), Centigram (CGM), Centimetre (CMT),
    /// Cubic inch (INQ), Cubic metre (MTQ), Fluid ounce (OZA), Foot (FOT), Hour (HUR), Item (ITM), Kilogram (KGM), Kilometre (KMT), Kilowatt (KWT), Liquid gallon (GLL), Liter (LTR), Pounds (LBS), Square foot (FTK).
    #[builder(default, setter(strip_option, into))]
    pub unit_of_measure: Option<String>,

    ///The item price or rate per unit.
    /// If you specify unit_amount, purchase_units[].amount.breakdown.item_total is required.
    /// Must equal unit_amount * quantity for all items.unit_amount.value can not be a negative number
    pub unit_amount: Amount,

    /// The item tax for each unit.
    /// If tax is specified, purchase_units[].amount.breakdown.tax_total is required.
    /// Must equal tax * quantity for all items.tax.value can not be a negative number.
    #[builder(default, setter(strip_option, into))]
    pub tax: Option<Amount>,

    /// Use this field to break down the discount amount included in the total purchase amount.
    /// The value provided here will not add to the total purchase amount. The value cannot be negative.
    #[builder(default, setter(strip_option, into))]
    pub discount_amount: Option<Amount>,

    /// The subtotal for all items. Must equal the sum of (items[].unit_amount * items[].quantity)
    /// for all items.item_total.value can not be a negative number.
    #[builder(default, setter(strip_option, into))]
    pub total_amount: Option<Amount>,
}
