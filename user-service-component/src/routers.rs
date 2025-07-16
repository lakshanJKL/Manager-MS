use spin_sdk::http::{IntoResponse, Request, Router};
use crate::controller::{create_user_controller, delete_user_controller, get_user_by_email_controller};

pub fn handle_route(req: Request) -> impl IntoResponse {
    let mut router = Router::new();
    router.post("/api/v1/users-service/create", create_user_controller);
    router.delete("/api/v1/users-service/delete/:id", delete_user_controller);
    router.get("/api/v1/users-service/get_user/:email", get_user_by_email_controller);
    router.handle(req)
}