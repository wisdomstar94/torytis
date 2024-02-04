use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};

use crate::statics::STATIC_DIR;

pub fn routes() -> Router {
    
    // let serve_dir = ServeDir::new(dir.path());
    // println!("@@serve_dir {:#?}", serve_dir);

    Router::new()
        // .route("/script.js", get(root_route))
        // .nest_service("/", serve_dir)
        .route("/content.css", get(content_css_route))
        .route("/postBtn.css", get(post_btn_css_route))
        .route("/another_category.css", get(another_category_css_route))
        

}

async fn content_css_route() -> Response {
    let file = STATIC_DIR.get_file("tistory-cdn/content.css").unwrap();
    let content = file.contents();
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/css")
      .body(Body::from(content))
      .unwrap();
}

async fn post_btn_css_route() -> Response {
    let file = STATIC_DIR.get_file("tistory-cdn/postBtn.css").unwrap();
    let content = file.contents();
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/css")
      .body(Body::from(content))
      .unwrap();
}

async fn another_category_css_route() -> Response {
    let file = STATIC_DIR.get_file("tistory-cdn/another_category.css").unwrap();
    let content = file.contents();
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/css")
      .body(Body::from(content))
      .unwrap();
}