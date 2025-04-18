use axum::{body::Body, extract::{Path, Request}, http::StatusCode, response::Response, routing::get, Router};
use serde::{Serialize, Deserialize};
use crate::{common::get_skin_html_content, structs::{replacer::{ApplyIndexListOptions, ApplyIndexPageOptions, ApplyPostPermalink, ApplyPostPermalinkPageOptions, Replacer}, torytis_dev_config::{PostSelectOption, TorytisDevConfig}}};

pub fn routes() -> Router {
    Router::new()
        .route("/{post_id}", get(post_permalink_route))
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

    let config = TorytisDevConfig::new();

    let mut is_show_home_cover = false;
    if let Some(vv) = config.get_skin_home_cover() {
        if let Some(kk) = vv.is_active {
            is_show_home_cover = kk;
        }
    }

    let skin_html_content = get_skin_html_content();
    let replacer = Replacer::new(&skin_html_content);
    replacer.apply_index_page(ApplyIndexPageOptions {
        search_keyword: String::from(""),
        base_url: format!(r#"/"#),
        body_id: String::from("tt-body-index"),
        is_show_home_cover,
        apply_index_list_option: ApplyIndexListOptions {
            is_no_render: is_show_home_cover,
            post_select_option: Some(PostSelectOption {
                page: Some(page),
                size: Some(size),
                post_type: None,
                category_name: None,
                sub_category_name: None,
                tag_name: None,
                title: None,
                post_id: None,
            }),
        },
    });
    
    return Response::builder()
      .status(StatusCode::OK)
      .header("Content-Type", "text/html")
      .body(Body::from(replacer.get_html()))
      .unwrap();
}

async fn post_permalink_route(Path(post_id): Path<String>, _: Request) -> Response {
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
