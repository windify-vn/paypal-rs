use crate::framework::auth::{ClientAccessToken, TokenStorage, TokenStorageProvider};
use crate::framework::client::ClientConfig;
use crate::framework::endpoint::{EndpointSpec, MultipartPart, RequestBody};
use crate::framework::response::ApiResponseType;
use crate::framework::{
    Environment, HeaderParams,
    auth::Credentials,
    response::ApiResponse,
    response::{ApiError, ApiFailure},
};
use std::borrow::Cow;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    environment: Environment,
    credentials: Credentials,
    token_storage: TokenStorageProvider,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(
        credentials: Credentials,
        config: ClientConfig,
        environment: Environment,
        token_storage: TokenStorageProvider,
    ) -> Result<Client, crate::framework::Error> {
        let mut builder = reqwest::Client::builder().default_headers(config.default_headers);

        #[cfg(not(target_arch = "wasm32"))]
        {
            // There is no resolve method in wasm.
            if let Some(address) = config.resolve_ip {
                let url = url::Url::from(&environment);
                builder = builder.resolve(
                    url.host_str()
                        .expect("Environment url should have a hostname"),
                    SocketAddr::new(address, 443),
                );
            }

            // There are no timeouts in wasm. The property is documented as no-op in wasm32.
            builder = builder.timeout(config.http_timeout);
        }

        let http_client = builder.build()?;

        Ok(Client {
            environment,
            credentials,
            token_storage,
            http_client,
        })
    }
    pub fn new_with_client(
        client: reqwest::Client,
        credentials: Credentials,
        environment: Environment,
        token_storage: TokenStorageProvider,
    ) -> Result<Client, crate::framework::Error> {
        Ok(Client {
            environment,
            credentials,
            token_storage,
            http_client: client,
        })
    }

    pub async fn request<Endpoint>(
        &self,
        endpoint: &Endpoint,
    ) -> ApiResponse<Endpoint::ResponseType>
    where
        Endpoint: EndpointSpec + Send + Sync,
    {
        self.request_with_headers(endpoint, &HeaderParams::default())
            .await
    }

    //noinspection RsConstantConditionIf
    /// Issue an API request of the given type.
    pub async fn request_with_headers<Endpoint>(
        &self,
        endpoint: &Endpoint,
        headers_params: &HeaderParams,
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
                    let mut form = reqwest::multipart::Form::new();
                    for (name, part) in multipart.parts() {
                        match part {
                            MultipartPart::Text(text) => {
                                form = form.text(name, text);
                            }
                            MultipartPart::Bytes(bytes) => {
                                form = form.part(name, reqwest::multipart::Part::bytes(bytes));
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

        if self.is_access_token_expired() {
            return Err(ApiFailure::AccessTokenExpired);
        }

        let token = match self.token_storage.token() {
            Some(token) => token,
            None => return Err(ApiFailure::AccessTokenExpired),
        };

        request = request.header("Authorization", format!("Bearer {token}"));

        let headers = headers_params.headers(&self.credentials);
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;

        let status = response.status();
        if status.is_success() {
            let full_bytes = response.bytes().await?;

            // let text = String::from_utf8_lossy(&full_bytes);
            // println!("{}", text);

            Endpoint::ResponseType::from_response(&full_bytes)
        } else {
            let parsed: Result<ApiError, reqwest::Error> = response.json().await;
            let errors = parsed.unwrap_or_default();
            Err(ApiFailure::Error(status, errors))
        }
    }

    pub async fn get_access_token(&mut self) -> Result<(), ApiFailure> {
        if !self.is_access_token_expired() {
            return Ok(());
        }

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

        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(ApiFailure::Error(
                response.status(),
                ApiError {
                    message: "Client Authentication failed".to_string(),
                    ..Default::default()
                },
            ));
        }

        let token: ClientAccessToken = response.json().await?;

        self.token_storage.set_token(token.access_token.clone());
        self.token_storage
            .set_expiry(Instant::now() + Duration::new(token.expires_in, 0));

        Ok(())
    }

    pub fn is_access_token_expired(&self) -> bool {
        self.token_storage.is_expired() || self.token_storage.token().is_none()
    }
}
