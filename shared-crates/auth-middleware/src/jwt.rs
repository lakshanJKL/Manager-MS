use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use hmac::{Mac, Hmac};
use sha2::Sha256;
use spin_sdk::variables;

type HmacSha256 = Hmac<Sha256>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub fn generate_token(email: &str) -> Result<String> {
    let secret_key = variables::get("secret").with_context(||
        "JWT SECRET must be set ".to_string()
    )?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: email.to_string(),
        exp: expiration,
    };

    let claims_json = serde_json::to_string(&claims)?;

    // Sign claims JSON using HMAC SHA256
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(claims_json.as_bytes());
    let signature = mac.finalize().into_bytes();

    // Build token as base64(claims_json) + "." + base64(signature)
    let token = format!(
        "{}.{}",
        general_purpose::STANDARD.encode(claims_json),
        general_purpose::STANDARD.encode(signature)
    );

    Ok(token)
}

pub fn verify_token(token: &str) -> Result<Claims> {
    let secret_key = variables::get("secret").with_context(||
        "JWT SECRET must be set ".to_string()
    )?;
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid token format");
    }

    let claims_json = general_purpose::STANDARD.decode(parts[0])?;
    let signature = general_purpose::STANDARD.decode(parts[1])?;

    // Recompute HMAC on claims_json
    let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(&claims_json);
    mac.verify_slice(&signature)
        .context("Invalid token signature")?;

    // Deserialize claims
    let claims: Claims = serde_json::from_slice(&claims_json)?;

    // Check expiration
    if Utc::now().timestamp() > claims.exp {
        anyhow::bail!("Token expired");
    }

    Ok(claims)
}