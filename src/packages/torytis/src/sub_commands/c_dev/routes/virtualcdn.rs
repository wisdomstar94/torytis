use std::fs;

use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};
use crate::{common::{get_torytis_dir_path_buf, get_working_dir_path_buf}, structs::replacer::Replacer};
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    let torytis_dir_path_buf = get_torytis_dir_path_buf();
    let serve_dir = ServeDir::new(torytis_dir_path_buf.as_path());

    Router::new()
        // .route("/script.js", get(root_route))
        .route("/style.css", get(style_css_route))
        .nest_service("/", serve_dir)
}

async fn style_css_route() -> Response {
    let style_css_path_buf = get_working_dir_path_buf().join(".torytis").join("style.css");
    let style_css_path = style_css_path_buf.as_path();

    let content = fs::read_to_string(style_css_path).unwrap();
    let replacer = Replacer::new(&content);
    replacer.apply_images_to_virtualcdn();

    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/css")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}