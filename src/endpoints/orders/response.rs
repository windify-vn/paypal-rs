use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTrackingResponse {
    pub id: String,
    pub status: String,
    pub purchase_units: Vec<PurchaseUnit>,
    pub links: Vec<HateoasLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseUnit {
    pub reference_id: String,
    pub items: Vec<OrderItem>,
    pub shipping: Option<Shipping>,
    pub payments: Option<Payments>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub name: String,
    pub sku: String,
    pub quantity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipping {
    pub trackers: Vec<Tracker>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracker {
    pub id: String,
    pub links: Vec<HateoasLink>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payments {
    pub captures: Vec<Capture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capture {
    pub id: String,
    pub status: String,
    pub amount: Amount,
    pub seller_protection: Option<SellerProtection>,
    pub final_capture: Option<bool>,
    pub seller_receivable_breakdown: Option<SellerReceivableBreakdown>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub links: Vec<HateoasLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amount {
    pub currency_code: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerProtection {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellerReceivableBreakdown {
    pub gross_amount: Amount,
    pub paypal_fee: Amount,
    pub net_amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HateoasLink {
    pub href: String,
    pub rel: String,
    pub method: String,
}

impl crate::framework::response::JsonResult for AddTrackingResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackResponse {
    pub id: String,
    pub purchase_units: Vec<CallbackPurchaseUnit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackPurchaseUnit {
    pub reference_id: String,
    pub amount: CallbackAmount,
    pub shipping_options: Vec<ShippingOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackAmount {
    pub currency_code: String,
    pub value: String,
    pub breakdown: Option<AmountBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountBreakdown {
    pub item_total: Option<Amount>,
    pub tax_total: Option<Amount>,
    pub shipping: Option<Amount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
    pub id: String,
    pub amount: ShippingAmount,
    #[serde(rename = "type")]
    pub shipping_type: String,
    pub label: String,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAmount {
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    pub value: String,
}

impl crate::framework::response::JsonResult for CallbackResponse {}
