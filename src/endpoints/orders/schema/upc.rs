use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone, Copy)]
#[allow(missing_docs)]
pub enum UniversalProductCodeType {
    #[serde(rename = "UPC-A")]
    UpcA,
    #[serde(rename = "UPC-B")]
    UpcB,
    #[serde(rename = "UPC-C")]
    UpcC,
    #[serde(rename = "UPC-D")]
    UpcD,
    #[serde(rename = "UPC-E")]
    UpcE,
    #[serde(rename = "UPC-2")]
    Upc2,
    #[serde(rename = "UPC-5")]
    Upc5,
}

/// The Universal Product Code of the item.
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UniversalProductCode {
    #[serde(rename = "type")]
    pub code_type: UniversalProductCodeType,
    #[builder(setter(into))]
    pub code: String,
}
