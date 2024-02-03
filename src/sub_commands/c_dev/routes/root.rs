use axum::{body::Body, extract::Request, http::StatusCode, response::Response, routing::get, Router};
use serde::{Serialize, Deserialize};
use crate::{common::get_skin_html_content, structs::{replacer::{ApplyGuestBookOptions, ApplyIndexListOptions, ApplyIndexPageOptions, ApplyTagListOptions, Replacer}, torytis_dev_config::PostSelectOption}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_route))
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
    replacer.apply_index_page(ApplyIndexPageOptions {
        base_url: format!(r#"/"#),
        body_id: String::from("tt-body-index"),
        apply_index_list_option: ApplyIndexListOptions {
            is_hide: false,
            post_select_option: Some(PostSelectOption {
                page: Some(page),
                size: Some(size),
                post_type: None,
                category_name: None,
                sub_category_name: None,
            }),
        },
        apply_guest_book_option: ApplyGuestBookOptions {
            is_hide: true,
        },
        apply_tag_list_option: ApplyTagListOptions {
            is_hide: true,
        },
    });
    
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}
