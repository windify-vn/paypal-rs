use crate::endpoints::orders::schema::address::{Address, PersonName, PhoneWithCountryCode};
use crate::endpoints::orders::schema::amount::Amount;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShippingType {
    /// The payer intends to receive the items at a specified address.
    Shipping,
    /// DEPRECATED. Please use "PICKUP_FROM_PERSON" instead.
    PickupInPerson,
    /// The payer intends to pick up the item(s) from the payee's physical store.
    /// Also termed as BOPIS, "Buy Online, Pick-up in Store". Seller protection is provided with this option.
    PickupInStore,
    /// The payer intends to pick up the item(s) from the payee in person.
    /// Also termed as BOPIP, "Buy Online, Pick-up in Person". Seller protection is not available, since the payer is receiving the item from the payee in person, and can validate the item prior to payment.
    PickupFromPerson,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Shipping {
    /// A classification for the method of purchase fulfillment (e.g shipping, in-store pickup, etc).
    /// Either type or options may be present, but not both.
    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "type")]
    pub shipping_type: Option<ShippingType>,

    /// An array of shipping options that the payee or merchant offers to the payer to ship or pick up their items.
    #[builder(default)]
    pub options: Vec<ShippingOption>,

    /// The name of the person to whom to ship the items. Supports only the full_name property.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<PersonName>,

    /// The email address of the recipient of the shipped items,
    /// which may belong to either the payer, or an alternate contact, for delivery.
    #[builder(default, setter(strip_option, into))]
    pub email_address: Option<String>,

    /// The phone number of the recipient of the shipped items, which may belong to either the payer,
    /// or an alternate contact, for delivery. [Format - canonical international E.164 numbering plan]
    #[builder(default, setter(strip_option, into))]
    pub phone_number: Option<PhoneWithCountryCode>,

    /// The address of the person to whom to ship the items. Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    /// admin_area_1 is required for addresses located in Argentina, Brazil, China, Canada, India, Indonesia, Japan, Mexico, Thailand, and the United States.
    #[builder(default, setter(strip_option, into))]
    pub address: Option<Address>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct ShippingOption {
    /// A unique ID that identifies a payer-selected shipping option.
    #[builder(setter(into))]
    pub id: String,

    /// A description that the payer sees, which helps them choose an appropriate shipping option.
    /// For example, Free Shipping, USPS Priority Shipping, Expédition prioritaire USPS, or USPS yōuxiān fā huò. Localize this description to the payer's locale.
    #[builder(setter(into))]
    pub label: String,

    /// If the API request sets selected = true, it represents the shipping option that the
    /// payee or merchant expects to be pre-selected for the payer when they first view the shipping.options in the PayPal Checkout experience.
    /// As part of the response if a shipping.option contains selected=true,
    /// it represents the shipping option that the payer selected during the course of checkout with PayPal. Only one shipping.option can be set to selected=true.
    #[builder(setter(strip_bool))]
    pub selected: bool,

    /// A classification for the method of purchase fulfillment.
    #[builder(default, setter(strip_option, into))]
    #[serde(rename = "type")]
    pub shipping_type: Option<ShippingType>,

    /// The shipping cost for the selected option.
    #[builder(default, setter(strip_option, into))]
    pub amount: Option<Amount>,
}
