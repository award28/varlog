use actix_web::{dev::ServiceRequest, web};

use anyhow::Result;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use hmac::Hmac;
use sha2::Sha256;
use jwt::{SignWithKey, VerifyWithKey};

use super::access_data::AccessData;

const APP_CONFIG_MISSING: &str = "🚨 APP CONFIG IS MISSING 🚨";
const BEARER_PREFIX: &str = "Bearer ";
const AUTH_HEADER: &str = "Authorization";
const DAY_IN_SECONDS: i64 = 86400;
#[derive(Debug, PartialEq)]
pub enum ClaimsError {
    InvalidAuthToken,
    NonBearerToken,
    NonASCIIHeader,
    NoAuthHeader,
    TokenExpired,
}

impl std::fmt::Display for ClaimsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ClaimsError::TokenExpired => "Token expired.",
            ClaimsError::NoAuthHeader => "No Authorization Header.",
            ClaimsError::NonASCIIHeader => "The Authorization header must be ASCII.",
            ClaimsError::NonBearerToken => "Only bearer tokens are supported.",
            ClaimsError::InvalidAuthToken => "Invalid authorization token.",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String,        // Claim: JWT, NanoID
    pub iat: i64,           // Claim: Issued At
    pub exp: i64,           // Claim: Expires
    pub data: AccessData,   // Granted Access
}

impl Claims {
    pub fn new(data: AccessData) -> Self {
        let jti = nanoid::nanoid!(6);
        let iat = chrono::offset::Utc::now().timestamp();
        let exp = iat + DAY_IN_SECONDS;

        Claims { jti, iat, exp, data }
    }

    pub fn sign(&self, key: &Hmac<Sha256>) -> Result<String> {
        Ok(self.clone().sign_with_key(key)?)
    }

    fn validate(&self) ->  Result<(), ClaimsError> {
        let now = Utc::now().timestamp();
        if now > self.exp {
            return Err(ClaimsError::TokenExpired);
        }

        Ok(())
    }
}

impl TryFrom<&ServiceRequest> for Claims {
    type Error = ClaimsError;

    fn try_from(req: &ServiceRequest) -> Result<Self, Self::Error> {
        let config: Option<&web::Data<crate::conf::AppConfig>> = req.app_data();

        // Hitting this case means that the app data is either misconfigured
        // or is being retrieved incorrectly.
        if config.is_none() {
            error!("{}", APP_CONFIG_MISSING);
            unreachable!();
        }

        let claims: Claims = req.headers()
            .get(AUTH_HEADER)
            .ok_or(ClaimsError::NoAuthHeader)?
            .to_str()
            .map_err(|_| ClaimsError::NonASCIIHeader)?
            .strip_prefix(BEARER_PREFIX)
            .ok_or(ClaimsError::NonBearerToken)?
            .verify_with_key(&config.unwrap().key)
            .map_err(|_| ClaimsError::InvalidAuthToken)?;

        claims.validate()?;
        Ok(claims)
    }
}
