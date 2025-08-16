use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Clone,
    Copy,
    strum::AsRefStr,
    strum::EnumString,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum Currency {
    /// Australian dollar
    AUD,
    /// Brazilian real, supported for in country paypal accounts only.
    BRL,
    /// Canadian dollar
    CAD,
    /// Chinese Renmenbi
    CNY,
    /// Czech koruna
    CZK,
    /// Danish krone
    DKK,
    /// Euro
    EUR,
    /// Hong Kong dollar
    HKD,
    /// Hungarian forint, does not support decimals.
    HUF,
    /// Indian rupee, supported for in country paypal india accounts only.
    INR,
    /// Israeli new shekel
    ILS,
    /// Japanese yen, does not support decimals.
    JPY,
    /// Malaysian ringgit
    MYR,
    /// Mexican peso
    MXN,
    /// New Taiwan dollar, does not support decimals.
    TWD,
    /// New Zealand dollar
    NZD,
    /// Norwegian krone
    NOK,
    /// Philippine peso
    PHP,
    /// Polish złoty
    PLN,
    /// Pound sterling
    GBP,
    /// Russian ruble
    RUB,
    /// Singapore dollar
    SGD,
    /// Swedish krona
    SEK,
    /// Swiss franc
    CHF,
    /// Thai baht
    THB,
    /// United States dollar
    #[default]
    USD,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Amount {
    /// The three-character ISO-4217 currency code that identifies the currency.
    #[builder(default)]
    pub currency_code: Currency,

    /// The value, which might be
    /// An integer for currencies like JPY that are not typically fractional.
    /// A decimal fraction for currencies like TND that are subdivided into thousandths.
    /// For the required number of decimal places for a currency code, see Currency Codes.
    #[builder(setter(into))]
    pub value: String,
}

impl From<(&str, Currency)> for Amount {
    fn from(value: (&str, Currency)) -> Self {
        Amount {
            currency_code: value.1,
            value: value.0.to_string(),
        }
    }
}

impl From<(f64, Currency)> for Amount {
    fn from(value: (f64, Currency)) -> Self {
        Amount {
            currency_code: value.1,
            value: format!("{:.2}", value.0),
        }
    }
}

impl From<(f32, Currency)> for Amount {
    fn from(value: (f32, Currency)) -> Self {
        Amount {
            currency_code: value.1,
            value: format!("{:.2}", value.0),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde_with::skip_serializing_none]
pub struct AmountBreakdown {
    #[serde(flatten)]
    pub amount: Amount,

    /// The breakdown of the amount.
    /// Breakdown provides details such as total item amount, total tax amount, shipping, handling, insurance, and discounts, if any.
    #[builder(default, setter(strip_option, into))]
    pub breakdown: Option<Breakdown>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct Breakdown {
    /// The subtotal for all items
    /// Must equal the sum of (items[].unit_amount * items[].quantity) for all items.
    /// item_total.value can not be a negative number.
    #[builder(default, setter(into))]
    pub item_total: Option<Amount>,

    /// The shipping fee for all items within a given purchase_unit.shipping.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub shipping: Option<Amount>,

    /// The handling fee for all items within a given purchase_unit.handling.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub handling: Option<Amount>,

    /// The total tax for all items. Required if the request includes purchase_units.items.tax.
    /// Must equal the sum of (items[].tax * items[].quantity) for all items.tax_total.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub tax_total: Option<Amount>,

    /// The insurance fee for all items within a given purchase_unit.insurance.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub insurance: Option<Amount>,

    /// The shipping discount for all items within a given purchase_unit.shipping_discount.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub shipping_discount: Option<Amount>,

    /// The discount for all items within a given purchase_unit.discount.value
    /// can not be a negative number.
    #[builder(default, setter(into))]
    pub discount: Option<Amount>,
}
