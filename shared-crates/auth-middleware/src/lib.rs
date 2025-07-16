use spin_sdk::http::{Headers, Request, Response};
use crate::jwt::{verify_token, Claims};
use anyhow::{Context, Result};
pub mod jwt;


pub fn authorize(req: &Request) -> Result<Claims> {

    let auth_header = req
        .headers()
        .find(|(k, _)| k.eq_ignore_ascii_case("authorization"))
        .context("Missing Authorization header")?
        .1
        .as_str()
        .context("Invalid header format")?
        .trim();

    if !auth_header.starts_with("Bearer ") {
        anyhow::bail!("Expected Bearer token");
    }

    let token = &auth_header[7..];
    verify_token(token)
}

pub fn unauthorized_response() -> Response {
    Response::builder()
        .status(401)
        .header("Content-Type", "application/json")
        .body("{\"error\": \"Unauthorized\"}")
        .build()
}