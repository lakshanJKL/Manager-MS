use spin_sdk::http::{IntoResponse, Request, Router};
use crate::controller::{create_image_controller, delete_image_controller, get_all_images_controller, get_image_controller, update_image_controller};

pub fn handle_router(req: Request) -> impl IntoResponse {
    let mut router = Router::new();
    router.post("/api/v1/image-service/create", create_image_controller);
    router.put("/api/v1/image-service/update/:id", update_image_controller);
    router.get("/api/v1/image-service/get-image/:id", get_image_controller);
    router.get("/api/v1/image-service/get_all_images", get_all_images_controller);
    router.delete("/api/v1/image-service/delete/:id", delete_image_controller);
    router.handle(req)
}