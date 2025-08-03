use crate::endpoints::orders::schema::source::apple_pay::ApplePaySource;
use crate::endpoints::orders::schema::source::bancontact::BanContactSource;
use crate::endpoints::orders::schema::source::blik::BlikSource;
use crate::endpoints::orders::schema::source::card::CardSource;
use crate::endpoints::orders::schema::source::eps::EpsSource;
use crate::endpoints::orders::schema::source::giropay::GiropaySource;
use crate::endpoints::orders::schema::source::google_pay::GooglePaySource;
use crate::endpoints::orders::schema::source::ideal::IdealSource;
use crate::endpoints::orders::schema::source::mybank::MybankSource;
use crate::endpoints::orders::schema::source::p24::P24Source;
use crate::endpoints::orders::schema::source::paypal::PaypalSource;
use crate::endpoints::orders::schema::source::pix::PixSource;
use crate::endpoints::orders::schema::source::sofort::SofortSource;
use crate::endpoints::orders::schema::source::swish::SwishSource;
use crate::endpoints::orders::schema::source::token::TokenSource;
use crate::endpoints::orders::schema::source::trustly::TrustlySource;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

pub mod apple_pay;
pub mod bancontact;
pub mod blik;
pub mod card;
pub mod eps;
pub mod giropay;
pub mod google_pay;
pub mod ideal;
pub mod mybank;
pub mod p24;
pub mod paypal;
pub mod pix;
pub mod sofort;
pub mod swish;
pub mod token;
pub mod trustly;
mod venmo;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PaymentSource {
    /// The payment card to use to fund a payment. Can be a credit or debit card.
    #[builder(default, setter(strip_option, into))]
    pub card: Option<CardSource>,

    /// The tokenized payment source to fund a payment.
    #[builder(default, setter(strip_option, into))]
    pub token: Option<TokenSource>,

    /// Indicates that PayPal Wallet is the payment source.
    /// Main use of this selection is to provide additional instructions associated with this choice like vaulting.
    #[builder(default, setter(strip_option, into))]
    pub paypal: Option<PaypalSource>,

    /// Bancontact is the most popular online payment in Belgium. More Details.
    #[builder(default, setter(strip_option, into))]
    pub bancontact: Option<BanContactSource>,

    /// BLIK is a mobile payment system, created by Polish Payment Standard in order to allow millions of users to pay in shops,
    /// payout cash in ATMs and make online purchases and payments. More Details.
    #[builder(default, setter(strip_option, into))]
    pub blik: Option<BlikSource>,

    /// The eps transfer is an online payment method developed by many Austrian banks. More Details.
    #[builder(default, setter(strip_option, into))]
    pub eps: Option<EpsSource>,

    /// Giropay is an Internet payment System in Germany, based on online banking. More Details.
    #[builder(default, setter(strip_option, into))]
    pub giropay: Option<GiropaySource>,

    /// The Dutch payment method iDEAL is an online payment method that enables consumers to pay online through their own bank. More Details.
    #[builder(default, setter(strip_option, into))]
    pub ideal: Option<IdealSource>,

    /// The Sweden payment method Swish is an online payment method that enables consumers to pay online through their own bank.
    #[builder(default, setter(strip_option, into))]
    pub swish: Option<SwishSource>,

    /// PIX is a local payment method in Brazil, that enables consumers to pay online through their own bank.
    #[builder(default, setter(strip_option, into))]
    pub pix: Option<PixSource>,

    /// MyBank is an e-authorisation solution which enables safe digital payments and identity
    /// authentication through a consumer’s own online banking portal or mobile application. More Details.
    #[builder(default, setter(strip_option, into))]
    pub mybank: Option<MybankSource>,

    /// P24 (Przelewy24) is a secure and fast online bank transfer service linked to all the major banks in Poland. More Details.
    #[builder(default, setter(strip_option, into))]
    pub p24: Option<P24Source>,

    /// SOFORT Banking is a real-time bank transfer payment method that buyers use to transfer funds directly to merchants from their bank accounts. More Details.
    #[builder(default, setter(strip_option, into))]
    pub sofort: Option<SofortSource>,

    /// Trustly is a payment method that allows customers to shop and pay from their bank account. More Details.
    #[builder(default, setter(strip_option, into))]
    pub trustly: Option<TrustlySource>,

    /// ApplePay payment source, allows buyer to pay using ApplePay, both on Web as well as on Native.
    #[builder(default, setter(strip_option, into))]
    pub apple_pay: Option<ApplePaySource>,

    /// Google Pay payment source, allows buyer to pay using Google Pay.
    #[builder(default, setter(strip_option, into))]
    pub google_pay: Option<GooglePaySource>,

    /// Information needed to indicate that Venmo is being used to fund the payment.
    #[builder(default, setter(strip_option, into))]
    pub venmo: Option<GooglePaySource>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaymentInitiator {
    /// Payment is initiated with the active engagement of the customer.
    /// e.g. a customer checking out on a merchant website.
    Customer,

    /// Payment is initiated by merchant on behalf of the customer without the active engagement of customer.
    /// e.g. a merchant charging the monthly payment of a subscription to the customer.
    Merchant,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum VaultStoreInValue {
    /// Defines that the payment_source will be vaulted only when at least one authorization or capture using that payment_source is successful.
    OnSuccess,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum ShippingPreference {
    /// Get the customer-provided shipping address on the PayPal site.
    #[default]
    GetFromFile,
    /// Removes the shipping address information from the API response and the Paypal site.
    /// However, the shipping.phone_number and shipping.email_address fields will still be returned to allow for digital goods delivery.
    NoShipping,
    /// Get the merchant-provided address.
    /// The customer cannot change this address on the PayPal site.
    /// If merchant does not pass an address, customer can choose the address on PayPal pages.
    SetProvidedAddress,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PaymentType {
    /// One Time payment such as online purchase or donation. (e.g. Checkout with one-click).
    OneTime,
    /// Payment which is part of a series of payments with fixed or variable amounts,
    /// following a fixed time interval. (e.g. Subscription payments).
    Recurring,
    /// Payment which is part of a series of payments that occur on a
    /// non-fixed schedule and/or have variable amounts. (e.g. Account Topup payments).
    Unscheduled,
}
