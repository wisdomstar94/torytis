use axum::{body::Body, extract::{Path, Request}, http::StatusCode, response::Response, routing::get, Router};
use serde::{Serialize, Deserialize};
use crate::{common::get_skin_html_content, structs::replacer::{ApplyTagPostIndexPageOptions, Replacer}};

pub fn routes() -> Router {
    Router::new()
        .route("/tag", get(root_route))
        .route("/tag/{tag_name}", get(tag_post_index_route))
        // .route("/:category_name", get(category_index_route))
        // .route("/:category_name/:sub_category_name", get(category_sub_category_index_route))
        // .route("/style.css", get(style_css_route))
}

#[derive(Debug, Serialize, Deserialize)]
struct RootPageQueryPayload {
    page: Option<u32>,
}

async fn root_route(_: Request) -> Response {
    let skin_html_content = get_skin_html_content();
    let replacer = Replacer::new(&skin_html_content);
    replacer.apply_tag_index_page();

    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}

async fn tag_post_index_route(Path(tag_name): Path<String>, req: Request) -> Response {
    let mut query_option: Option<RootPageQueryPayload> = None;
    if let Some(query_str) = req.uri().query() {
        if let Ok(payload) = serde_qs::from_str::<RootPageQueryPayload>(query_str) {
          query_option = Some(payload);
        } else {
          println!("query_str : {:?}", query_str);
        }
    }

    let size = 8;
    let page = if let Some(q) = query_option {
        q.page.unwrap_or_else(|| 1)
    } else {
        1
    };

    let skin_html_content = get_skin_html_content();
    let replacer = Replacer::new(&skin_html_content);
    replacer.apply_tag_post_index_page(ApplyTagPostIndexPageOptions {
        tag_name,
        page,
        size,
    });

    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}