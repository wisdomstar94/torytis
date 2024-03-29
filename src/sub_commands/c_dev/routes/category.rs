use axum::{body::Body, extract::{Path, Request}, http::StatusCode, response::Response, routing::get, Router};
use serde::{Serialize, Deserialize};
use crate::{common::get_skin_html_content, structs::{replacer::{ApplyIndexListOptions, ApplyIndexPageOptions, Replacer}, torytis_dev_config::PostSelectOption}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_route))
        .route("/:category_name", get(category_index_route))
        .route("/:category_name/:sub_category_name", get(category_sub_category_index_route))
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
        search_keyword: String::from(""),
        base_url: format!(r#"/category"#),
        body_id: String::from("tt-body-category"),
        is_show_home_cover: false,
        apply_index_list_option: ApplyIndexListOptions {
            is_hide: false,
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

async fn category_index_route(Path(category_name): Path<String>, req: Request) -> Response {
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
        search_keyword: String::from(""),
        base_url: format!(r#"/category/{}"#, category_name),
        body_id: String::from("tt-body-category"),
        is_show_home_cover: false,
        apply_index_list_option: ApplyIndexListOptions {
            is_hide: false,
            post_select_option: Some(PostSelectOption {
                page: Some(page),
                size: Some(size),
                post_type: None,
                category_name: Some(category_name.clone()),
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

async fn category_sub_category_index_route(Path((category_name, sub_category_name)): Path<(String, String)>, req: Request) -> Response {
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
        search_keyword: String::from(""),
        base_url: format!(r#"/category/{}/{}"#, category_name, sub_category_name),
        body_id: String::from("tt-body-category"),
        is_show_home_cover: false,
        apply_index_list_option: ApplyIndexListOptions {
            is_hide: false,
            post_select_option: Some(PostSelectOption {
                page: Some(page),
                size: Some(size),
                post_type: None,
                category_name: Some(category_name.clone()),
                sub_category_name: Some(sub_category_name.clone()),
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


// async fn style_css_route() -> Response {
//     let content = get_style_css_content();

//     // let skin_html_content_str = skin_html_content.as_str();
//     return Response::builder()
//       .status(StatusCode::OK)
//       .header("Content-Type", "text/css")
//       .body(Body::from(content))
//       .unwrap();
// }