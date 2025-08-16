use dyn_clone::DynClone;
use serde::Deserialize;
use std::fmt::Debug;
use std::time::{Duration, Instant};

pub trait TokenStorage: DynClone + Debug {
    fn token(&self) -> Option<String>;
    fn set_token(&mut self, token: String);

    fn expiry(&self) -> Option<Instant>;
    fn set_expiry(&mut self, expiry: Instant);

    fn is_expired(&self) -> bool;
}

dyn_clone::clone_trait_object!(TokenStorage);

#[derive(Clone, Debug)]
pub enum TokenStorageProvider {
    Memory {
        token: String,
        expiry: Option<Instant>,
    },
    External {
        storage: Box<dyn TokenStorage + Send + Sync>,
    },
}

impl TokenStorage for TokenStorageProvider {
    fn token(&self) -> Option<String> {
        match self {
            Self::Memory { token, .. } => Some(token.clone()),
            Self::External { storage } => storage.token(),
        }
    }

    fn set_token(&mut self, token: String) {
        match self {
            Self::Memory {
                token: current_token,
                ..
            } => *current_token = token,
            Self::External { storage } => storage.set_token(token),
        }
    }

    fn expiry(&self) -> Option<Instant> {
        match self {
            Self::Memory { expiry, .. } => *expiry,
            Self::External { storage } => storage.expiry(),
        }
    }

    fn set_expiry(&mut self, expiry: Instant) {
        match self {
            Self::Memory {
                expiry: current_expiry,
                ..
            } => *current_expiry = Some(expiry),
            Self::External { storage } => storage.set_expiry(expiry),
        }
    }

    fn is_expired(&self) -> bool {
        match self {
            Self::Memory { expiry, .. } => {
                if let Some(expiry) = expiry {
                    Instant::now() >= *expiry
                } else {
                    true
                }
            }
            Self::External { storage } => storage.is_expired(),
        }
    }
}

impl TokenStorageProvider {
    pub fn new_memory() -> Self {
        Self::Memory {
            token: "".to_string(),
            expiry: None,
        }
    }

    pub fn new_external(storage: Box<dyn TokenStorage + Send + Sync>) -> Self {
        Self::External { storage }
    }

    pub fn time_until_expiry(&self) -> Option<Duration> {
        match self.expiry() {
            Some(expiry) => {
                let now = Instant::now();
                if now < expiry {
                    Some(expiry - now)
                } else {
                    Some(Duration::ZERO)
                }
            }
            None => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ClientAccessToken {
    pub(crate) scope: String,
    pub(crate) access_token: String,
    pub(crate) token_type: String,
    pub(crate) app_id: String,
    pub(crate) expires_in: u64,
    pub(crate) nonce: String,
}

#[derive(Clone, Debug)]
pub struct Credentials {
    pub(crate) client_id: String,
    pub(crate) secret: String,
}

impl Credentials {
    pub fn new(client_id: String, secret: String) -> Self {
        Self { client_id, secret }
    }
}
