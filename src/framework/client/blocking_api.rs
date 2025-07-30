use crate::framework::auth::{ClientAccessToken, Credentials, TokenStorage, TokenStorageProvider};
use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::{ApiError, ApiFailure, ApiResponse, ApiResponseType};
use crate::framework::{Environment, HeaderParams};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use url::Url;

pub struct HttpApiClient {
    environment: Environment,
    credentials: Credentials,
    token_storage: TokenStorageProvider,
    http_client: reqwest::blocking::Client,
}

impl HttpApiClient {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
        token_storage: TokenStorageProvider,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        let mut builder = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers);

        if let Some(address) = config.resolve_ip {
            let url = url::Url::from(&environment);
            builder = builder.resolve(
                url.host_str()
                    .expect("Environment url should have a hostname"),
                SocketAddr::new(address, 443),
            );
        }
        let http_client = builder.build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            token_storage,
            http_client,
        })
    }

    pub fn new_with_client(
        client: reqwest::blocking::Client,
        credentials: Credentials,
        environment: Environment,
        token_storage: TokenStorageProvider,
    ) -> Result<HttpApiClient, crate::framework::Error> {
        Ok(HttpApiClient {
            environment,
            credentials,
            token_storage,
            http_client: client,
        })
    }

    pub fn request<Endpoint>(&mut self, endpoint: &Endpoint) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
    {
        self.request_with_headers(endpoint, &HeaderParams::default())
    }

    pub fn request_with_headers<Endpoint>(
        &mut self,
        endpoint: &Endpoint,
        header_params: &HeaderParams,
    ) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(endpoint.method(), endpoint.url(&self.environment));

        if let Some(body) = endpoint.body() {
            match body {
                RequestBody::Json(json) => {
                    request = request.body(json);
                }
                RequestBody::Raw(bytes) => {
                    request = request.body(bytes);
                }
                RequestBody::MultiPart(multipart) => {
                    let mut form = reqwest::blocking::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form
                                    .part(name, reqwest::blocking::multipart::Part::bytes(bytes));
                            }
                        }
                    }
                    request = request.multipart(form);
                }
            }
            // Reqwest::RequestBuilder::multipart sets the content type for us.
            match endpoint.content_type() {
                None | Some(Cow::Borrowed("multipart/form-data")) => {}
                Some(content_type) => {
                    request = request.header(reqwest::header::CONTENT_TYPE, content_type.as_ref());
                }
            }
        }

        let token = self.token_storage.token();
        let token = if self.token_storage.is_expired() || token.is_none() {
            let request = self
                .http_client
                .request(
                    http::Method::POST,
                    Url::from(&self.environment)
                        .join("v1/oauth2/token")
                        .unwrap(),
                )
                .basic_auth(&self.credentials.client_id, Some(&self.credentials.secret))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body("grant_type=client_credentials");

            let response = request.send()?;
            if response.status() != http::StatusCode::OK {
                return Err(ApiFailure::Error(
                    response.status(),
                    ApiError {
                        message: "Client Authentication failed".to_string(),
                        ..Default::default()
                    },
                ));
            }

            let token: ClientAccessToken = response.json()?;

            self.token_storage.set_token(token.access_token.clone());
            self.token_storage
                .set_expiry(Instant::now() + Duration::new(token.expires_in, 0));

            token.access_token
        } else {
            token.unwrap_or_default()
        };

        request = request.header("Authorization", format!("Bearer {token}"));

        let headers = header_params.headers(&self.credentials);
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send()?;

        let status = response.status();
        if status.is_success() {
            let full_bytes = response.bytes()?;

            // let text = String::from_utf8_lossy(&full_bytes);
            // println!("{}", text);

            Endpoint::ResponseType::from_response(&full_bytes)
        } else {
            let parsed: Result<ApiError, reqwest::Error> = response.json();
            let errors = parsed.unwrap_or_default();
            Err(ApiFailure::Error(status, errors))
        }
    }
}
