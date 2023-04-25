use actix_web::{dev::ServiceRequest, web, Scope};


use serde::{Serialize, Deserialize};
use hmac::Hmac;
use sha2::Sha256;
use jwt::{SignWithKey, error::Error, VerifyWithKey};
use regex::RegexSet;

const DAY_IN_SECONDS: i64 = 86400;

pub mod middleware;
pub mod routes;

pub fn service() -> Scope {
    web::scope("")
        .service(routes::register)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessData {
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

impl AccessData {
    pub fn file_access_authorized(&self, filename: &String) -> bool {
        RegexSet::new(self.paths.clone()).unwrap().matches(filename.as_str()).matched_any()
    }

    pub fn server_access_authorized(&self, server: &str) -> bool {
        RegexSet::new(self.servers.clone()).unwrap().matches(server).matched_any()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub jti: String, // Claim: JWT, NanoID
    pub iat: i64, // Claim: Issued At
    pub exp: i64, // Claim: Expires
    pub data: AccessData,
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

    pub fn sign(key: &Hmac<Sha256>, data: AccessData) -> Result<(Self, String), Error> {
        let claim = Self::new(data);
        Ok((claim.clone(), claim.sign_with_key(key)?))
    }

    fn validate(&self) -> Result<(), String> {
        let now = chrono::offset::Utc::now().timestamp();
        if now > self.exp {
            return Err(String::from("Token expired."));
        }

        Ok(())
    }
}

impl TryFrom<&ServiceRequest> for Claims {
    type Error = String;

    fn try_from(req: &ServiceRequest) -> Result<Self, Self::Error> {
        let config: Option<&web::Data<crate::conf::AppConfig>> = req.app_data();

        // Hitting this case means that the app data is either misconfigured
        // or is being retrieved incorrectly.
        if config.is_none() {
            return Err(String::from("Internal Server Error."));
        }

        let claims: Claims = req.headers()
            .get("Authorization")
            .ok_or("No Authorization header.")?
            .to_str()
            .map_err(|_| "The Authorization header must be ASCII.")?
            .strip_prefix("Bearer ")
            .ok_or("Only bearer tokens are supported.")?
            .verify_with_key(&config.unwrap().key)
            .map_err(|_| "Invalid authorization token.")?;

        claims.validate()?;
        Ok(claims)
    }
}
