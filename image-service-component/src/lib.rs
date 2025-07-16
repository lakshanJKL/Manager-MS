use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

mod persistence;
mod routers;
mod model;
mod controller;

#[http_component]
fn handle_image_service_component(req: Request) -> anyhow::Result<impl IntoResponse> {
  Ok(routers::handle_router(req))
}
