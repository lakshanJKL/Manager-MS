use anyhow::{anyhow, Context, Result};
use crate::bcrypt::{hashed_password, verify_password};
use crate::model::{LoginModel, SignupModel, UserResponse};
use spin_sdk::http::{Request as SpinRequest, Method, send, Response};
use auth_middleware::jwt::generate_token;

pub async fn signup(dto: &SignupModel) -> Result<Response> {
    let hashed_pw = hashed_password(&dto.password)?;

    let user_dto = SignupModel {
        email: dto.email.clone(),
        full_name: dto.full_name.clone(),
        gender: dto.gender.clone(),
        contact: dto.contact.clone(),
        password: hashed_pw,
    };

    let json = serde_json::to_vec(&user_dto)?;

    let req = SpinRequest::builder()
        .method(Method::Post)
        .uri("https://managers-api.fermyon.app/api/v1/users-service/create")
        .header("Content-Type", "application/json")
        .body(json)
        .build();

    let res = send(req)
        .await
        .context("Failed to forward signup to User Service")?;

    Ok(res)
}

pub async fn login(dto: LoginModel) -> Result<String> {
    let get_user_by_email_url = format!(
        "https://managers-api.fermyon.app/api/v1/users-service/get_user/{}",
        dto.email.clone()
    );

    let req = SpinRequest::builder()
        .method(Method::Get)
        .uri(&get_user_by_email_url)
        .header("Content-Type", "application/json")
        .body(vec![])
        .build();

    let res: Response = send(req)
        .await
        .context("Failed to contact User Service")?;

    let status = res.status();
    if !(200u16..300u16).contains(status) {
        return Err(anyhow!("User not found or error from user service"));
    }


    let bytes = res.body();
    let user: UserResponse = serde_json::from_slice(bytes)
        .context("Failed to parse user response")?;

    let is_verified = verify_password(&dto.password, &user.password)?;
    if !is_verified {
        return Err(anyhow!("Invalid password"));
    }

    let token = generate_token(&dto.email)?;
    Ok(token)
}
