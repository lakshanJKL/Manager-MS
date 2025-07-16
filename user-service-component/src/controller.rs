use anyhow::Result;
use spin_sdk::http::{Request, IntoResponse, Response, Params};
use crate::model::CreateUserModel;
use crate::persistence::{create_user, delete_user_by_id, find_user_by_email};

pub fn create_user_controller(req: Request, _param: Params) -> Result<impl IntoResponse> {
    let Ok(user_dto) = serde_json::from_slice::<CreateUserModel>(req.body())
    else {
        return Ok(Response::new(400, "not found data"));
    };
    create_user(&user_dto)?;
    Ok(Response::builder()
        .status(201)
        .body("User created")
        .build()
    )
}

pub fn delete_user_controller(_req: Request, param: Params) -> Result<impl IntoResponse> {
    let Some(id) = param.get("id")
    else {
        return Ok(Response::new(400, "Invalid id"));
    };

    delete_user_by_id(id)?;
    Ok(Response::builder()
        .status(204)
        .body("user deleted")
        .build()
    )
}

pub fn get_user_by_email_controller(_req: Request, param: Params) -> Result<impl IntoResponse> {
    let Some(email) = param.get("email")
    else {
        return Ok(Response::new(400, "Invalid email"));
    };

    match find_user_by_email(email)? {
        Some(user)=>{
            let json =serde_json::to_string(&user)?;
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(json)
                .build()
            )
        },
        None=> Ok(Response::new(404, "User not found"))
    }
}