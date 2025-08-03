use crate::endpoints::orders::schema::address::{Address, PhoneWithCountryCode};
use crate::endpoints::orders::schema::source::card::{CardNetwork, CardType};
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePaySource {
    /// Name on the account holder associated with Google Pay.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    /// The email address of the account holder associated with Google Pay.
    #[builder(default, setter(strip_option, into))]
    pub email_address: Option<String>,

    /// The phone number of account holder, in its canonical international E.164 numbering plan format.
    /// Supports only the national_number property.
    #[builder(default, setter(strip_option, into))]
    pub phone_number: Option<PhoneWithCountryCode>,

    /// The payment card information.
    #[builder(default, setter(strip_option, into))]
    pub card: Option<GooglePayPaymentCard>,

    /// The decrypted payload details for the Google Pay token.
    #[builder(default, setter(strip_option, into))]
    pub decrypted_token: Option<GooglePayDecryptedToken>,

    /// Information about what validation has been performed on the returned payment credentials.
    #[builder(default, setter(strip_option, into))]
    pub assurance_details: Option<GooglePayAssuranceDetails>,

    /// Customizes the payer experience during the approval process for the payment.
    #[builder(default, setter(strip_option, into))]
    pub experience_context: Option<GooglePayExperienceContext>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePayPaymentCard {
    /// The card holder's name as it appears on the card.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    /// The payment card type.
    #[serde(rename = "type")]
    #[builder(default, setter(strip_option, into))]
    pub card_type: Option<CardType>,

    /// The card brand or network. Typically used in the response.
    #[builder(default, setter(strip_option, into))]
    pub brand: Option<CardNetwork>,

    /// The billing address for this card.
    /// Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    #[builder(default, setter(strip_option, into))]
    pub billing_address: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize, Default, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum GooglePayDecryptTokenPaymentMethod {
    /// CARD is the only value that Google Pay accepts.
    #[default]
    Card,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum GooglePayAuthenticationMethod {
    /// This authentication method is associated with payment cards stored on file with the user's Google Account.
    /// Returned payment data includes primary account number (PAN) with the expiration month and the expiration year.
    PanOnly,
    /// Returned payment data includes a 3-D Secure (3DS) cryptogram generated on the device.
    /// -> If authentication_method=CRYPTOGRAM, it is required that 'cryptogram' parameter in the request has a valid 3-D Secure (3DS) cryptogram generated on the device.
    #[serde(rename = "CRYPTOGRAM_3DS")]
    Cryptogram3ds,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePayDecryptedToken {
    /// A unique ID that identifies the message in case it needs to be revoked or located at a later time.
    #[builder(default, setter(strip_option, into))]
    pub message_id: Option<String>,

    /// Date and time at which the message expires as UTC milliseconds since epoch.
    /// Integrators should reject any message that's expired.
    #[builder(default, setter(strip_option, into))]
    pub message_expiration: Option<String>,

    /// The type of the payment credential. Currently, only CARD is supported.
    #[builder(default, setter(into))]
    pub payment_method: GooglePayDecryptTokenPaymentMethod,

    /// Authentication Method which is used for the card transaction.
    #[builder(setter(into))]
    pub authentication_method: GooglePayAuthenticationMethod,

    /// Base-64 cryptographic identifier used by card schemes to validate the token verification result.
    /// This is a conditionally required field if authentication_method is CRYPTOGRAM_3DS.
    #[builder(default, setter(strip_option, into))]
    pub cryptogram: Option<String>,

    /// Electronic Commerce Indicator may not always be present.
    /// It is only returned for tokens on the Visa card network. This value is passed through in the payment authorization request.
    #[builder(default, setter(strip_option, into))]
    pub eci_indicator: Option<String>,

    /// Google Pay tokenized credit card used to pay.
    #[builder(setter(into))]
    pub card: GooglePayDecryptedTokenCard,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePayDecryptedTokenCard {
    /// The card holder's name as it appears on the card.
    #[builder(default, setter(strip_option, into))]
    pub name: Option<String>,

    /// The primary account number (PAN) for the payment card.
    #[builder(setter(into))]
    pub number: String,

    /// The card expiration year and month, in Internet date format.
    #[builder(setter(into))]
    pub expiry: String,

    /// The payment card type.
    #[serde(rename = "type")]
    #[builder(default, setter(strip_option, into))]
    pub card_type: Option<CardType>,

    /// The card brand or network. Typically used in the response.
    #[builder(default, setter(strip_option, into))]
    pub brand: Option<CardNetwork>,

    /// The billing address for this card.
    /// Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    #[builder(default, setter(strip_option, into))]
    pub billing_address: Option<Address>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePayAssuranceDetails {
    /// If true, indicates that Cardholder possession validation has been performed on returned payment credential.
    #[builder(setter(strip_bool))]
    pub account_verified: bool,

    /// If true, indicates that identification and verifications (ID&V) was performed on the returned payment credential.
    /// If false, the same risk-based authentication can be performed as you would for card transactions.
    /// This risk-based authentication can include, but not limited to, step-up with 3D Secure protocol if applicable.
    #[builder(setter(strip_bool))]
    pub card_holder_authenticated: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct GooglePayExperienceContext {
    /// The URL where the customer is redirected after the customer approves the payment.
    #[builder(setter(into))]
    pub return_url: String,

    /// The URL where the customer is redirected after the customer cancels the payment.
    #[builder(setter(into))]
    pub cancel_url: String,
}
