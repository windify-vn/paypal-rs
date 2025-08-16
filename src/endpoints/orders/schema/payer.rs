use crate::endpoints::orders::schema::address::{Address, PersonFullName, PhoneWithType};
use crate::endpoints::orders::schema::source::paypal::PaypalTaxInfo;
use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Payer {
    /// The email address of the payer.
    pub email_address: Option<String>,

    /// The PayPal-assigned ID for the payer.
    pub payer_id: Option<String>,

    /// The name of the payer. Supports only the given_name and surname properties.
    pub name: Option<PersonFullName>,

    /// The phone number of the customer.
    /// Available only when you enable the Contact Telephone Number option in the Profile & Settings for the merchant's PayPal account.
    /// The phone.phone_number supports only national_number.
    pub phone: Option<PhoneWithType>,

    /// The birth date of the payer in YYYY-MM-DD format.
    pub birth_date: Option<String>,

    /// The tax information of the payer. Required only for Brazilian payer's. Both tax_id and tax_id_type are required.
    pub tax_payer: Option<PaypalTaxInfo>,

    /// The address of the payer.
    /// Supports only the address_line_1, address_line_2, admin_area_1, admin_area_2, postal_code, and country_code properties.
    /// Also referred to as the billing address of the customer.
    pub address: Option<Address>,
}
