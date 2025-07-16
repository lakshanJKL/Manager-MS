use spin_sdk::http::{IntoResponse, Params, Request, Response};
use anyhow::Result;
use crate::model::CreateImageModel;
use crate::persistence::{create_image, delete_image_by_id, get_all_images, get_image_by_id, update_image_by_id};
use auth_middleware::{authorize, unauthorized_response};
pub fn create_image_controller(req: Request, _param: Params) -> Result<impl IntoResponse> {

    // verify authorization
    if let Err(_) =  authorize(&req) {
        return Ok(unauthorized_response());
    }

    let Ok(image_dto) = serde_json::from_slice::<CreateImageModel>(req.body())
    else {
        return Ok(Response::new(400, "Data not found"));
    };
    create_image(&image_dto)?;
    Ok(Response::builder()
        .status(201)
        .body("Image created")
        .build())
}

pub fn update_image_controller(req: Request, param: Params) -> Result<impl IntoResponse> {

    // verify authorization
    if let Err(_) =  authorize(&req) {
        return Ok(unauthorized_response());
    }

    let Some(id) = param.get("id")
    else {
        return Ok(Response::new(400, "Invalid id"));
    };

    let Ok(dto) = serde_json::from_slice::<CreateImageModel>(req.body())
    else {
        return Ok(Response::new(400, "Data not found"));
    };
    update_image_by_id(id, dto)?;
    Ok(Response::builder()
        .status(201)
        .body("image updated")
        .build()
    )
}

pub fn get_all_images_controller(req: Request, _param: Params) -> Result<impl IntoResponse> {

    // verify authorization
    if let Err(_) =  authorize(&req) {
        return Ok(unauthorized_response());
    }

    let images = get_all_images()?;
    let json = serde_json::to_string(&images)?;

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(json)
        .build()
    )
}
pub fn get_image_controller(req: Request, param: Params) -> Result<impl IntoResponse> {

    // verify authorization
    if let Err(_) =  authorize(&req) {
        return Ok(unauthorized_response());
    }

    let Some(id) = param.get("id")
    else {
        return Ok(Response::new(400, "Invalid no"));
    };

    match get_image_by_id(id) {
        Ok(image) => {
            let json = serde_json::to_string(&image)?;
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body(json)
                .build()
            )
        }
        Err(_) => Ok(Response::new(500, "Internal Server Error"))
    }
}

pub fn delete_image_controller(req: Request, param: Params) -> Result<impl IntoResponse> {

    // verify authorization
    if let Err(_) =  authorize(&req) {
        return Ok(unauthorized_response());
    }

    let Some(id) = param.get("id")
    else {
        return Ok(Response::new(400, "Invalid id"));
    };
    delete_image_by_id(id)?;
    Ok(Response::builder()
        .status(204)
        .body("image deleted")
        .build()
    )
}