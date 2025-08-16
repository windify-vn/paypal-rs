use crate::framework::auth::Credentials;
use base64::Engine;
use serde::Serialize;
use typed_builder::TypedBuilder;

pub mod auth;
pub mod client;
pub mod endpoint;
pub mod response;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error via the `reqwest` crate
    #[error("Reqwest returned an error when connecting to the Paypal API: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

/// Which environment (host path) to use for API calls
#[derive(Debug, Clone)]
pub enum Environment {
    Live,
    Sandbox,
    Custom(String),
}

impl From<&Environment> for url::Url {
    fn from(environment: &Environment) -> Self {
        match environment {
            Environment::Live => url::Url::parse("https://api-m.paypal.com/").unwrap(),
            Environment::Sandbox => url::Url::parse("https://api-m.sandbox.paypal.com/").unwrap(),
            Environment::Custom(url) => url::Url::parse(url.as_str()).unwrap(),
        }
    }
}

/// Represents the optional header values used on paypal requests.
///
/// <https://developer.paypal.com/api/rest/requests/>
#[derive(Debug, Default, Clone, TypedBuilder)]
pub struct HeaderParams {
    /// The merchant payer id used on PayPal-Auth-Assertion
    #[builder(default, setter(strip_option, into))]
    pub merchant_payer_id: Option<String>,
    /// Verifies that the payment originates from a valid, user-consented device and application.
    /// Reduces fraud and decreases declines. Transactions that do not include a client metadata ID are not eligible for PayPal Seller Protection.
    #[builder(default, setter(strip_option, into))]
    pub client_metadata_id: Option<String>,
    /// Identifies the caller as a PayPal partner. To receive revenue attribution, specify a unique build notation (BN) code.
    /// BN codes provide tracking on all transactions that originate or are associated with a particular partner.
    #[builder(default, setter(strip_option, into))]
    pub partner_attribution_id: Option<String>,
    /// Contains a unique user-generated ID that the server stores for a period of time. Use this header to enforce idempotency on REST API POST calls.
    /// You can make these calls any number of times without concern that the server creates or completes an action on a resource more than once.
    /// You can retry calls that fail with network timeouts or the HTTP 500 status code. You can retry calls for as long as the server stores the ID.
    #[builder(default, setter(strip_option, into))]
    pub request_id: Option<String>,
    /// Simulate an error. Pass an error code to test through the mock_application_codes header parameter.
    #[builder(default, setter(strip_option, into))]
    pub mock_response: Option<String>,
    /// The server returns a minimal response to optimize communication between the API caller and the server
    #[builder(setter(strip_bool))]
    pub prefer_minimal: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct AuthAssertionClaims {
    pub(crate) iss: String,
    pub(crate) payer_id: String,
}

impl HeaderParams {
    pub fn headers(&self, credentials: &Credentials) -> Vec<(&'static str, String)> {
        let mut ret = vec![];

        if let Some(merchant_payer_id) = &self.merchant_payer_id {
            let claims = AuthAssertionClaims {
                iss: credentials.client_id.clone(),
                payer_id: merchant_payer_id.clone(),
            };

            let jwt_header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
            let token = jsonwebtoken::encode(
                &jwt_header,
                &claims,
                &jsonwebtoken::EncodingKey::from_secret(credentials.secret.as_bytes()),
            )
            .unwrap_or_default();

            let encoded_token = base64::engine::general_purpose::STANDARD.encode(token);

            ret.push(("PayPal-Auth-Assertion", encoded_token))
        }

        if let Some(client_metadata_id) = &self.client_metadata_id {
            ret.push(("PayPal-Client-Metadata-Id", client_metadata_id.clone()))
        }

        if let Some(partner_attribution_id) = &self.partner_attribution_id {
            ret.push((
                "PayPal-Partner-Attribution-Id",
                partner_attribution_id.clone(),
            ))
        }

        if let Some(request_id) = &self.request_id {
            ret.push(("PayPal-Request-Id", request_id.clone()))
        }

        if let Some(mock_response) = &self.mock_response {
            ret.push(("PayPal-Mock-Response", mock_response.clone()))
        }

        if self.prefer_minimal {
            ret.push(("Prefer", "return=minimal".to_string()))
        } else {
            ret.push(("Prefer", "return=representation".to_string()))
        }

        ret
    }
}
