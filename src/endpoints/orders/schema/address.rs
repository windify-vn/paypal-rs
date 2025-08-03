use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(missing_docs)]
pub enum PhoneType {
    /// Fax number.
    Fax,
    /// Home phone number.
    Home,
    /// Mobile phone number.
    Mobile,
    /// Other phone number.
    Other,
    /// Pager number.
    Pager,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct PersonName {
    /// When the party is a person, the party's full name.
    #[builder(setter(into))]
    pub full_name: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct PersonFullName {
    /// When the party is a person, the party's given, or first, name.
    pub given_name: Option<String>,
    /// When the party is a person, the party's surname or family name.
    /// Also known as the last name. Required when the party is a person. Use also to store multiple surnames including the matronymic, or mother's, surname.
    pub surname: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option, into)))]
pub struct Address {
    /// The first line of the address, such as number and street, for example, 173 Drury Lane.
    /// Needed for data entry, and Compliance and Risk checks. This field needs to pass the full address.
    pub address_line_1: Option<String>,
    /// The second line of the address, for example, a suite or apartment number.
    pub address_line_2: Option<String>,
    /// A city, town, or village. Smaller than admin_area_level_1.
    pub admin_area_2: Option<String>,
    /// The highest-level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision.
    /// This data is formatted for postal delivery, for example, CA and not California.
    pub admin_area_1: Option<String>,
    /// The postal code, which is the ZIP code or equivalent. Typically required for countries with a postal code or an equivalent. See postal code.
    pub postal_code: Option<String>,
    /// The 2-character ISO 3166-1 code that identifies the country or region.
    pub country_code: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, TypedBuilder)]
pub struct PhoneWithCountryCode {
    /// The country calling code (CC), in its canonical international E.164 numbering plan format.
    /// The combined length of the CC and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    #[builder(default, setter(strip_option, into))]
    pub country_code: Option<String>,
    /// The national number, in its canonical international E.164 numbering plan format.
    /// The combined length of the country calling code (CC) and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    #[builder(setter(into))]
    pub national_number: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, TypedBuilder)]
pub struct PhoneWithType {
    /// The phone type.
    #[builder(default, setter(strip_option, into))]
    pub phone_type: Option<PhoneType>,

    /// The phone number, in its canonical international E.164 numbering plan format.
    /// Supports only the national_number property.
    #[builder(setter(into))]
    pub phone_number: PhoneNumber,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default, Clone, TypedBuilder)]
pub struct PhoneNumber {
    /// The national number, in its canonical international E.164 numbering plan format.
    /// The combined length of the country calling code (CC) and the national number must not be greater than 15 digits.
    /// The national number consists of a national destination code (NDC) and subscriber number (SN).
    #[builder(setter(into))]
    pub national_number: String,
}
