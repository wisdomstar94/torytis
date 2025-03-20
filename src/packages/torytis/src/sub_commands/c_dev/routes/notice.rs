use axum::{body::Body, extract::{Path, Request}, http::StatusCode, response::Response, routing::get, Router};
use serde::{Serialize, Deserialize};
use crate::{common::get_skin_html_content, structs::replacer::{ApplyNoticeIndexPageOptions, ApplyPostPermalink, ApplyPostPermalinkPageOptions, Replacer}};

pub fn routes() -> Router {
    Router::new()
        .route("/notice", get(root_route))
        .route("/notice/{post_id}", get(notice_permalink_route))
        // .route("/:category_name", get(category_index_route))
        // .route("/:category_name/:sub_category_name", get(category_sub_category_index_route))
        // .route("/style.css", get(style_css_route))
}

#[derive(Debug, Serialize, Deserialize)]
struct RootPageQueryPayload {
    page: Option<u32>,
}

async fn root_route(req: Request) -> Response {
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

    replacer.apply_notice_index_page(ApplyNoticeIndexPageOptions {
        page,
        size,
    });

    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}

async fn notice_permalink_route(Path(post_id): Path<String>, _: Request) -> Response {
    let skin_html_content = get_skin_html_content();
    let replacer = Replacer::new(&skin_html_content);
    replacer.apply_post_permalink_page(ApplyPostPermalinkPageOptions {
        apply_post_permalink: Some(ApplyPostPermalink {
            post_id,
        }),
    });
    
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}
