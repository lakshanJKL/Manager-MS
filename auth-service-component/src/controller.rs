use spin_sdk::http::{IntoResponse, Params, Request, Response};
use anyhow::Result;
use serde_json::json;
use crate::model::{LoginModel, SignupModel};
use crate::persistence::{login, signup};

pub async fn signup_controller(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let Ok(dto) = serde_json::from_slice::<SignupModel>(req.body())
    else {
       return  Ok(Response::new(400, "Invalid format data "))
    };

    let res = signup(&dto).await?;

    let status = res.status();
    if !(200..300).contains(status) {
        return Ok(Response::new(500, "User creation failed"));
    }

    Ok(Response::builder()
        .status(201)
        .body("User signed up successfully")
        .build()
    )
}

pub async fn login_controller(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let Ok(dto) = serde_json::from_slice::<LoginModel>(req.body())
    else {
        return Ok(Response::new(400, "Invalid format data"));
    };

    let token = login(dto).await?;
    let json = json!({ "token": format!("Bearer {token}") }).to_string();
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json)
        .build())
}