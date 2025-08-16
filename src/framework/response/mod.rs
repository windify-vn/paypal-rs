mod api_fail;

pub use api_fail::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ApiSuccess<ResultType> {
    #[serde(flatten)]
    pub result: ResultType,
    #[serde(default)]
    pub links: Vec<HateoasLink>,
}

impl<T> JsonResult for ApiSuccess<T> where T: JsonResult {}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, Debug)]
pub struct HateoasLink {
    pub href: String,
    pub rel: String,
    pub method: Option<String>,
}

pub type ApiResponse<ResultType> = Result<ResultType, ApiFailure>;

pub trait JsonResult: DeserializeOwned + Debug {}

pub trait ApiResponseType: Sized {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure>;
}
impl<T> ApiResponseType for T
where
    T: JsonResult,
{
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        serde_json::from_slice(bytes).map_err(ApiFailure::Decoding)
    }
}

impl ApiResponseType for String {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        let text = String::from_utf8_lossy(bytes);

        Ok(text.into_owned())
    }
}

impl ApiResponseType for Vec<u8> {
    fn from_response(bytes: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(bytes.to_vec())
    }
}

impl ApiResponseType for () {
    fn from_response(_: &bytes::Bytes) -> Result<Self, ApiFailure> {
        Ok(())
    }
}
