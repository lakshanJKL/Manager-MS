use spin_sdk::http::{IntoResponse, Request};
use spin_sdk::http_component;

mod model;
mod controller;
mod routers;
mod persistence;

#[http_component]
fn handle_user_service_component(req: Request) -> anyhow::Result<impl IntoResponse> {
  Ok(routers::handle_route(req))
}
