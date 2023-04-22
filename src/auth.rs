use actix_web::dev::ServiceRequest;
use chrono;

use serde::{Serialize, Deserialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::{SignWithKey, error::Error, VerifyWithKey};

const DAY_IN_SECONDS: i64 = 86400;

pub mod middleware;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessData {
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    jti: String, // Claim: JWT, NanoID
    iat: i64, // Claim: Issued At
    exp: i64, // Claim: Expires
    data: AccessData,
}

impl Claims {
    pub fn new(data: AccessData) -> Self {
        let jti = nanoid::nanoid!(6);
        let iat = chrono::offset::Utc::now().timestamp();
        let exp = iat + DAY_IN_SECONDS;
        Claims {
            jti,
            iat,
            exp,
            data,
        }
    }

    pub fn signed(data: AccessData) -> Result<String, Error> {
        // TODO: What do I do with this key??
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"horse-battery-staple-gun")
            .expect("Key should be parsable");
        Self::new(data).sign_with_key(&key)
    }
}

impl TryFrom<&ServiceRequest> for Claims {
    type Error = String;

    fn try_from(req: &ServiceRequest) -> Result<Self, Self::Error> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"horse-battery-staple-gun")
            .expect("Key should be parsable");

        let claims: Claims = req.headers()
            .get("Authorization")
            .ok_or("No Authorization header.")?
            .to_str()
            .map_err(|_| "The Authorization header must be ASCII.")?
            .strip_prefix("Bearer ")
            .ok_or("Only bearer tokens are supported.")?
            .verify_with_key(&key)
            .map_err(|_| "Invalid authorization token.")?;

        Ok(claims)
    }
}
