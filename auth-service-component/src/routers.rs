use spin_sdk::http::{IntoResponse, Request, Router};
use crate::controller::{login_controller, signup_controller};

pub fn handle_router(req: Request) -> impl IntoResponse {
    let mut router = Router::new();
    router.post_async("/auth/signup", signup_controller);
    router.post_async("/auth/login", login_controller);
    router.handle(req)
}