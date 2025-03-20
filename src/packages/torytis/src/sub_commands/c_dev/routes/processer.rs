use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};
use crate::functions::{build_preprocess::build_preprocess, move_public_to_dot_torytis::move_public_to_dot_torytis, script_bundle::script_bundle, script_postprocess::script_postprocess, skin_html_replace::skin_html_replace};

pub fn routes() -> Router {
    Router::new()
        // .route("/script.js", get(root_route))
        .route("/processer/buildpreprocess", get(buildpreprocess_route))
        .route("/processer/movepublictodottorytis", get(movepublictodottorytis_route))
        .route("/processer/scriptbundle", get(scriptbundle_route))
        .route("/processer/scriptpostprocess", get(scriptpostprocess_route))
        .route("/processer/skinhtmlreplace", get(skinhtmlreplace_route))
        
}

async fn buildpreprocess_route() -> Response {
  build_preprocess(&false);

  return Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html")
    .body(Body::from("ok"))
    .unwrap();
}

async fn movepublictodottorytis_route() -> Response {
  move_public_to_dot_torytis(&false);

  return Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html")
    .body(Body::from("ok"))
    .unwrap();
}

async fn scriptbundle_route() -> Response {
  script_bundle();

  return Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html")
    .body(Body::from("ok"))
    .unwrap();
}

async fn scriptpostprocess_route() -> Response {
  script_postprocess(&false);

  return Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html")
    .body(Body::from("ok"))
    .unwrap();
}

async fn skinhtmlreplace_route() -> Response {
  skin_html_replace(&true);

  return Response::builder()
    .status(StatusCode::OK)
    .header("Content-Type", "text/html")
    .body(Body::from("ok"))
    .unwrap();
}

