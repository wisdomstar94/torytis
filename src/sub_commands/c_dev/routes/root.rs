use axum::{Router, routing::get, response::Response, http::StatusCode, body::Body};

use crate::{common::get_skin_html_content, structs::{replacer::Replacer, torytis_dev_config::TorytisDevConfig}};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root_route))
        // .route("/style.css", get(style_css_route))
}

async fn root_route() -> Response {
    let skin_html_content = get_skin_html_content();
    let replacer = Replacer::new(&skin_html_content).apply_index_page();
    // skin_html_content = replace_s_search(&skin_html_content, "");
    // skin_html_content = replace_common(&config, &skin_html_content);
    // skin_html_content = replace_home_display(&config, &skin_html_content);
    // skin_html_content = replace_var_page_title(&skin_html_content, config.get_blog_title());
    // skin_html_content = replace_var_body_id(&skin_html_content, "tt-body-index");

    // let skin_html_content_str = skin_html_content.as_str();
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