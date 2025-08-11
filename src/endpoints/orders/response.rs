use crate::endpoints::orders::schema::PaymentIntent;
use crate::endpoints::orders::schema::source::PaymentSource;
use crate::endpoints::orders::schema::unit::PurchaseUnit;
use crate::framework::response::HateoasLink;
use crate::framework::response::JsonResult;
use serde::Deserialize;

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct CreateOrderResponse {
    pub id: String,
    pub intent: Option<PaymentIntent>,
    pub status: Option<String>,
    pub purchase_units: Vec<PurchaseUnit>,
    pub payment_source: Option<PaymentSource>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    #[serde(default)]
    pub links: Vec<HateoasLink>,
}
impl JsonResult for CreateOrderResponse {}

pub type ShowOrderDetailResponse = CreateOrderResponse;
pub type ConfirmOrderResponse = CreateOrderResponse;
pub type AuthorizeOrderResponse = CreateOrderResponse;
pub type CaptureOrderResponse = CreateOrderResponse;
#[derive(Debug, Deserialize)]
pub struct UpdateOrderResponse;

impl JsonResult for UpdateOrderResponse {}
