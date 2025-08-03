use crate::endpoints::orders::schema::address::PhoneWithCountryCode;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PixSource {
    /// The 3-character ISO-4217 currency code that identifies the currency.
    #[builder(setter(into))]
    pub currency_code: String,

    /// QR details received from processor.
    #[builder(default, setter(strip_option, into))]
    pub qr_details: Option<PixQrDetails>,

    /// The two-character ISO 3166-1 purchase country code.
    #[builder(setter(into))]
    pub country_code: String,

    /// The name of the account holder associated with PIX.
    #[builder(setter(into))]
    pub name: String,

    /// The email address of the account holder associated with Pix.
    #[builder(setter(into))]
    pub email_address: String,

    /// The phone number, in its canonical international E.164 numbering plan format. Supports only the country_code and national_number properties.
    #[builder(setter(into))]
    pub phone_number: PhoneWithCountryCode,

    #[builder(setter(into))]
    pub tax_info: PixTaxInfo,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PixQrDetails {
    /// QR Expiry time in seconds
    #[builder(default, setter(strip_option, into))]
    pub qr_expiry: Option<String>,

    /// QR image received from APM processor,
    /// The pattern is not provided because the value is defined by an external party.
    #[builder(default, setter(strip_option, into))]
    pub qr_image: Option<String>,

    /// QR payload received from APM processor,
    /// The pattern is not provided because the value is defined by an external party.
    #[builder(default, setter(strip_option, into))]
    pub qr_payload: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PixTaxIdType {
    /// The individual tax ID type, typically is 11 characters long.
    BrCpf,

    /// The business tax ID type, typically is 14 characters long.
    BrCnpj,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PixTaxInfo {
    /// The customer's tax ID value.
    #[builder(setter(into))]
    pub tax_id: String,

    /// The customer's tax ID type.
    #[builder(setter(into))]
    pub tax_id_type: PixTaxIdType,
}
