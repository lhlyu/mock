use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

mod api_images;
use api_images::images_router;

fn router() -> Router {
    Router::new().route("/", get(root)).merge(images_router())
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "一切皆有来处，一切终有归途。我们无从知晓，我们无需害怕，命运自有安排。"
}
