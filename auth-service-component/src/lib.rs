use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

mod bcrypt;
mod controller;
mod routers;
mod persistence;
mod model;

#[http_component]
fn handle_auth_service_component(req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(routers::handle_router(req))
}
