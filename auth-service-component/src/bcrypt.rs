use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hashed_password(password: &String) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    let verified = verify(password, hashed_password)?;
    Ok(verified)
}