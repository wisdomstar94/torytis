use axum::Router;
use crate::common::get_torytis_dir_path_buf;
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    let torytis_dir_path_buf = get_torytis_dir_path_buf();
    let serve_dir = ServeDir::new(torytis_dir_path_buf.as_path());

    Router::new()
        // .route("/script.js", get(root_route))
        .nest_service("/", serve_dir)
}

// async fn root_route() -> Response {
//     let content = get_script_js_content();

//     // let skin_html_content_str = skin_html_content.as_str();
//     return Response::builder()
//       .status(StatusCode::OK)
//       .header("Content-Type", "text/css")
//       .body(Body::from(content))
//       .unwrap();
// }