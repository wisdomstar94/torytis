use std::{cell::RefCell, ops::Deref, rc::Rc};
use html_regex::Bucket;

use crate::structs::torytis_dev_config::TorytisDevConfig;

pub struct Replacer {
    html: Rc<RefCell<String>>,
    config: TorytisDevConfig,
}

impl Replacer {
    pub fn new(html: &str) -> Self {
        Self {
            html: Rc::new(RefCell::new(html.to_owned())),
            config: TorytisDevConfig::new(),
        }
    }

    pub fn get_html(&self) -> String {
        self.html.deref().borrow().to_string()
    }

    pub fn get_torytis_dev_config(&self) -> &TorytisDevConfig {
        &self.config
    }
}

impl Replacer {
    pub fn apply_index_page(&self) -> Self {
        let self_html_borrow_mut = self.html.deref().borrow_mut();
        let html = self_html_borrow_mut.deref();
        let root = Bucket::new(html);
        Self::new(&root.get_html())
    }
}





// use chrono::NaiveDateTime;
// use regex::Regex;
// use crate::structs::torytis_dev_config::{PostType, TorytisDevConfig};

// pub fn replace_common(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);
//     result = replace_style_css_url(&result);
//     result = replace_script_js_url(&result);
//     result = replace_s_rctrp_rep(config, &result);
//     result = replace_s_rct_notice(config, &result);
//     result = replace_s_sidebar(&result);
//     result = replace_var_title(config, &result);
//     result = replace_var_category_list(config, &result);
//     result = replace_var_visitor(config, &result);
//     result = replace_var_revenue_list_upper(&result);
//     result = replace_var_revenue_list_lower(&result);
//     result
// }

// fn replace_s_sidebar(html: &str) -> String {
//     let mut result = String::from(html);
//     let regex = Regex::new(r#"<s_sidebar>(.|\n)*?</s_sidebar>"#).unwrap();
//     let result_str: &String = &result.clone();

//     let matched_string_iter = regex.find_iter(result_str).map(|s| -> String {
//         let s_sidebar_block = s.as_str();
//         s_sidebar_block.to_string()
//     });

//     for s_sidebar_block in matched_string_iter {
//         let s_sidebar_element_inner_html_vec = get_s_sidebar_element_inner_html_vec(&s_sidebar_block);
//         let mut new_content = String::new();
//         for item in s_sidebar_element_inner_html_vec {
//             new_content.push_str(item.as_str());
//         }
//         result = result.replacen(s_sidebar_block.as_str(), &new_content, 1);
//     }

//     result  
// }

// fn get_s_sidebar_element_inner_html_vec(s_sidebar_block: &str) -> Vec<String> {
//     let mut vec = Vec::new();

//     let regex = Regex::new(r#"<s_sidebar_element>(.|\n)*?</s_sidebar_element>"#).unwrap();

//     let matched_string_iter = regex.find_iter(s_sidebar_block).map(|s| -> String {
//         s.as_str().to_string()
//     });

//     for s_sidebar_element_block in matched_string_iter {
//         vec.push(s_sidebar_element_block.replace("<s_sidebar_element>", "").replace("</s_sidebar_element>", ""));
//     }

//     vec
// }

// pub fn replace_var_page_title(html: &str, str: Option<&str>) -> String {
//     let mut result = String::from(html);
//     if let Some(v) = str {
//         result = result.replace(r#"[##_page_title_##]"#, v);
//     }
//     result
// }

// pub fn replace_var_body_id(html: &str, str: &str) -> String {
//     let mut result = String::from(html);
//     result = result.replace(r#"[##_body_id_##]"#, str);
//     result
// }

// fn replace_var_title(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);
//     if let Some(v) = config.get_blog_title() {
//         result = result.replace(r#"[##_title_##]"#, v);
//     }
//     result
// }

// pub fn replace_var_category_list(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);

//     let mut li_tag_html_list: Vec<String> = Vec::new();
//     let mut is_onece_new_exist = false;
//     if let Some(category_list) = config.get_category_list() {
//         for parent_category in category_list {
//             if let Some(child_category_list) = &parent_category.category_list {
//                 // 자식 카테고리가 있는 경우
//                 let mut child_category_li_tag_html_list: Vec<String> = Vec::new();
//                 let mut is_child_exist_new = false;
//                 for child_category in child_category_list {
//                     let mut new_tag = "";
//                     if let Some(is_new) = child_category.is_new {
//                         if is_new {
//                             is_child_exist_new = true;
//                             is_onece_new_exist = true;
//                             new_tag = r#"<img alt="N" src="https://tistory1.daumcdn.net/tistory_admin/blogs/image/category/new_ico_5.gif" style="vertical-align:middle;padding-left:2px;" />"#;
//                         }
//                     }
//                     child_category_li_tag_html_list.push(format!(
//                         r#"
//                             <li class="">
//                                 <a href="/category/{}/{}" class="link_sub_item"> 
//                                     {} <span class="c_cnt">(10)</span> 
//                                     {}
//                                 </a>
//                             </li>
//                         "#, 
//                         parent_category.name, 
//                         child_category.name, 
//                         child_category.name,
//                         new_tag
//                     ));
//                 }
//                 let child_category_ul_html = format!(
//                     r#"
//                         <ul class="sub_category_list">
//                             {}
//                         </ul>
//                     "#, 
//                     child_category_li_tag_html_list.join("")
//                 );

//                 let mut new_tag = "";
//                 if let Some(is_new) = parent_category.is_new {
//                     if is_new {
//                         is_onece_new_exist = true;
//                         new_tag = r#"<img alt="N" src="https://tistory1.daumcdn.net/tistory_admin/blogs/image/category/new_ico_5.gif" style="vertical-align:middle;padding-left:2px;" />"#;
//                     }
//                 }
//                 if is_child_exist_new {
//                     is_onece_new_exist = true;
//                     new_tag = r#"<img alt="N" src="https://tistory1.daumcdn.net/tistory_admin/blogs/image/category/new_ico_5.gif" style="vertical-align:middle;padding-left:2px;" />"#;
//                 }
//                 li_tag_html_list.push(format!(
//                     r#"
//                         <li class="">
//                             <a href="/category/{}" class="link_item"> 
//                                 {} <span class="c_cnt">(25)</span> 
//                                 {}
//                             </a>
//                             {}
//                         </li>
//                     "#, 
//                     parent_category.name, 
//                     parent_category.name, 
//                     new_tag,
//                     child_category_ul_html
//                 ));
//             } else {
//                 // 자식 카테고리가 없는 경우
//                 let mut new_tag = "";
//                 if let Some(is_new) = parent_category.is_new {
//                     if is_new {
//                         is_onece_new_exist = true;
//                         new_tag = r#"<img alt="N" src="https://tistory1.daumcdn.net/tistory_admin/blogs/image/category/new_ico_5.gif" style="vertical-align:middle;padding-left:2px;" />"#;
//                     }
//                 }
//                 li_tag_html_list.push(format!(
//                     r#"
//                         <li class="">
//                             <a href="/category/{}" class="link_item"> 
//                                 {} <span class="c_cnt">(4)</span> 
//                                 {}
//                             </a>
//                         </li>
//                     "#, 
//                     parent_category.name, 
//                     parent_category.name,
//                     new_tag,
//                 ));
//             }
//         } 
//     }
    
//     let mut total_new_tag = "";
//     if is_onece_new_exist {
//         total_new_tag = r#"<img alt="N" src="https://tistory1.daumcdn.net/tistory_admin/blogs/image/category/new_ico_5.gif" style="vertical-align:middle;padding-left:2px;" />"#;
//     }
//     let category_list_html = format!(
//         r#"
//             <ul class="tt_category">
//                 <li class="">
//                     <a href="/category" class="link_tit"> 
//                         분류 전체보기 <span class="c_cnt">(209)</span> 
//                         {}
//                     </a>
//                     <ul class="category_list">
//                         {}
//                     </ul>
//                 </li>
//             </ul>
//         "#, 
//         total_new_tag,
//         li_tag_html_list.join("")
//     );

//     result = result.replace(r#"[##_category_list_##]"#, &category_list_html);
//     result
// }

// pub fn replace_var_visitor(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);
//     let visitor = config.get_visitor();
//     if let Some(v) = visitor {
//         if let Some(x) = v.count_total {
//             result = result.replace(r#"[##_count_total_##]"#, x.to_string().as_str());
//         }
//         if let Some(x) = v.count_today {
//             result = result.replace(r#"[##_count_today_##]"#, x.to_string().as_str());
//         }
//         if let Some(x) = v.count_yesterday {
//             result = result.replace(r#"[##_count_yesterday_##]"#, x.to_string().as_str());
//         }
//     }
//     result
// }

// fn replace_var_revenue_list_upper(html: &str) -> String {
//     let mut result = String::from(html);
//     result = result.replace(r#"[##_revenue_list_upper_##]"#, "");
//     result
// }

// fn replace_var_revenue_list_lower(html: &str) -> String {
//     let mut result = String::from(html);
//     result = result.replace(r#"[##_revenue_list_lower_##]"#, "");
//     result
// }

// pub fn replace_style_css_url(html: &str) -> String {
//     let mut result = String::from(html);
//     result = result.replace(r#"<link href="./style.css" type="text/css" rel="stylesheet" />"#, r#"<link href="/virtualcdn/style.css" type="text/css" rel="stylesheet" />"#);
//     result
// }

// pub fn replace_script_js_url(html: &str) -> String {
//     let mut result = String::from(html);
//     result = result.replace(r#"<script src="./images/script.js"></script>"#, r#"<script src="/virtualcdn/images/script.js"></script>"#);
//     result
// }

// pub fn replace_s_rctrp_rep(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);
//     if let Some(recent_comment_list) = config.get_recent_comment_list() {
//         let items = get_tag_block_list(&result, "s_sidebar_element", Some("s_rctrp_rep"), None);
//         for item in items {
//             let s_sidebar_element_block_original = item.as_str();
//             let mut s_sidebar_element_block = s_sidebar_element_block_original.to_string();
//             let regex_s_rctrp_rep_block = Regex::new(r#"<s_rctrp_rep>((.|\n)*?)</s_rctrp_rep>"#).unwrap();
//             if let Some(m) = regex_s_rctrp_rep_block.find(&s_sidebar_element_block) {
//                 let s_rctrp_rep_block = m.as_str(); // "<s_rctrp_rep>......</s_rctrp_rep>"
//                 let s_rctrp_rep_block_inner_html = s_rctrp_rep_block.replace(r#"<s_rctrp_rep>"#, "").replace(r#"</s_rctrp_rep>"#, "");
//                 let mut li_html_list: Vec<String> = Vec::new();
//                 for item in &recent_comment_list {
//                     let mut html = s_rctrp_rep_block_inner_html.clone();
//                     if let Some(v) = &item.name {
//                         html = html.replace(r#"[##_rctrp_rep_name_##]"#, v);
//                     }
//                     if let Some(v) = &item.datetime {
//                         html = html.replace(r#"[##_rctrp_rep_time_##]"#, NaiveDateTime::parse_from_str(&v, "%Y-%m-%d %H:%M:%S").unwrap().format("%m.%d").to_string().as_str());
//                     }
//                     if let Some(v) = &item.content {
//                         html = html.replace(r#"[##_rctrp_rep_desc_##]"#, v);
//                     }
//                     if let Some(v) = &item.comment_id {
//                         let post_id = config.get_post_id_from_comment_id(v).unwrap_or_else(|| String::new());
//                         html = html.replace(r#"[##_rctrp_rep_link_##]"#, format!("/{}#{}", post_id, v).as_str());
//                     }
//                     li_html_list.push(html);
//                 }
//                 // regex_s_rctrp_rep_block.replace(s_sidebar_element_block, s_rctrp_rep_block_inner_html);
//                 s_sidebar_element_block = regex_s_rctrp_rep_block.replace(&s_sidebar_element_block, &li_html_list.join("")).to_string();
//             }
//             result = result.replace(s_sidebar_element_block_original, &s_sidebar_element_block);
//             // result = result.replace(from, to)
//         }
//     }
//     result
// }

// pub fn replace_s_rct_notice(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);
//     let notice_posts = config.get_posts(Some(&PostType::Notice)).unwrap_or_else(|| vec![]);
//     let s_sidebar_block_list = get_tag_block_list(html, "s_sidebar_element", Some("s_rct_notice"), None);
//     for s_sidebar_block in s_sidebar_block_list {
//         let mut s_sidebar_block_new = s_sidebar_block.clone();
//         let s_rct_notice_block_list = get_tag_block_list(&s_sidebar_block_new, "s_rct_notice", None, None);
//         for s_rct_notice_block in s_rct_notice_block_list {
//             let mut s_rct_notice_block_new = s_rct_notice_block.clone().replace(r#"<s_rct_notice>"#, "").replace(r#"</s_rct_notice>"#, "");
//             let s_rct_notice_rep_block_list = get_tag_block_list(&s_rct_notice_block_new, "s_rct_notice_rep", None, None);
//             for s_rct_notice_rep_block in s_rct_notice_rep_block_list {
//                 let li_template_block = s_rct_notice_rep_block.clone().replace(r#"<s_rct_notice_rep>"#, "").replace(r#"</s_rct_notice_rep>"#, "");
//                 let mut li_list: Vec<String> = Vec::new();
//                 for notice_post in &notice_posts {
//                     let mut li_template_block_copy = li_template_block.clone();
//                     li_template_block_copy = li_template_block_copy.replace(r#"[##_notice_rep_title_##]"#, notice_post.title.clone().unwrap_or_else(|| String::new()).as_str());
//                     li_template_block_copy = li_template_block_copy.replace(r#"[##_notice_rep_link_##]"#, format!("/notice/{}", notice_post.post_id.clone().unwrap_or_else(|| String::new()).as_str()).as_str());
//                     li_list.push(li_template_block_copy);
//                 }
//                 let li_list_html = li_list.join("");
//                 s_rct_notice_block_new = s_rct_notice_block_new.replace(&s_rct_notice_block_new, &li_list_html);
//             }
//             s_sidebar_block_new = s_sidebar_block_new.replace(s_rct_notice_block.as_str(), s_rct_notice_block_new.as_str());
//         }
//         result = result.replacen(s_sidebar_block.as_str(), s_sidebar_block_new.as_str(), 1);
//     }
//     result
// }

// pub fn replace_s_search(html: &str, search: &str) -> String {
//     let mut result = String::from(html);
//     let s_sidebar_block_list = get_tag_block_list(html, "s_sidebar_element", Some("s_search"), None);
//     for s_sidebar_block in s_sidebar_block_list {
//         let mut s_sidebar_block_new = s_sidebar_block.clone();
//         let s_search_block_list = get_tag_block_list(&s_sidebar_block_new, "s_search", None, None);
//         for s_search_block in s_search_block_list {
//             let mut s_search_block_new = s_search_block.clone().replace(r#"<s_search>"#, "").replace(r#"</s_search>"#, "");
//             s_search_block_new = s_search_block_new.replace(r#"[##_search_name_##]"#, r#"search"#);
//             s_search_block_new = s_search_block_new.replace(r#"[##_search_text_##]"#, search);
//             s_search_block_new = s_search_block_new.replace(r#"[##_search_onclick_submit_##]"#, r#"
//                 try {
//                     window.location.href = '/search' + '/' + encodeURI(document.getElementsByName('search')[0].value);
//                     document.getElementsByName('search')[0].value = '';
//                     return false;
//                 } catch (e) {
                    
//                 } 
//             "#);
//             s_sidebar_block_new = s_sidebar_block_new.replace(s_search_block.as_str(), s_search_block_new.as_str());
//         }
//         result = result.replacen(s_sidebar_block.as_str(), s_sidebar_block_new.as_str(), 1);
//     }
//     result
// }

// fn get_tag_block_list(html: &str, target_tag_name: &str, require_child_tag_name: Option<&str>, is_attr_exist: Option<bool>) -> Vec<String> {
//     let is_attr_exist_unwrap = is_attr_exist.unwrap_or_else(|| false);
//     let target_tag_block_regex = if is_attr_exist_unwrap {
//         Regex::new(format!(r#"<{}\s+(.*?)>((.|\n)*?)</{}>"#, target_tag_name, target_tag_name).as_str()).unwrap()
//     } else {
//         Regex::new(format!(r#"<{}>((.|\n)*?)</{}>"#, target_tag_name, target_tag_name).as_str()).unwrap()
//     };
//     let tag_block_list = target_tag_block_regex.find_iter(html).filter(|s| -> bool {
//         let mut is_allow = true;
//         if let Some(k) = require_child_tag_name {
//             let pattern = format!(r#"<{}>((.|\n)*?)</{}>"#, k, k);
//             let regex = Regex::new(&pattern).unwrap();
//             is_allow = regex.is_match(s.as_str());
//         }
//         is_allow
//     }).map(|s| -> String {
//         s.as_str().to_string()
//     }).collect::<Vec<String>>();
//     tag_block_list
// }

// pub fn replace_home_display(config: &TorytisDevConfig, html: &str) -> String {
//     let mut result = String::from(html);

//     // if is_empty {
//     //     let s_cover_group_tag_block_list = get_tag_block_list(&result, "s_cover_group", None);
//     //     for item in s_cover_group_tag_block_list {
//     //         result = result.replace(item.as_str(), "");
//     //     }
//     //     let s_notice_rep_tag_block_list = get_tag_block_list(&result, "s_notice_rep", None);
//     //     for item in s_notice_rep_tag_block_list {
//     //         result = result.replace(item.as_str(), "");
//     //     }
//     //     let s_article_rep_tag_block_list = get_tag_block_list(&result, "s_article_rep", None);
//     //     for item in s_article_rep_tag_block_list {
//     //         result = result.replace(item.as_str(), "");
//     //     }
//     //     return result;
//     // }

//     if let None = config.get_skin_home_cover() {
//         return result;
//     }

//     let skin_home_cover = config.get_skin_home_cover().unwrap();
//     if skin_home_cover.is_active.unwrap_or_else(|| false) {
//         // 홈 화면을 커버 아이템 화면으로..
//         let xml_cover_items = config.get_xml_cover_items();
//         println!("@xml_cover_items : {:#?}", xml_cover_items);
//         for cover_item in skin_home_cover.cover_items.unwrap_or_else(|| vec![]) {
//             let name = cover_item.cover_name.unwrap();
//             let title = cover_item.cover_title.unwrap();
//             let category_name = cover_item.cover_category_name.unwrap();
//             let posts = config.get_posts_from_category_name(category_name.as_str());
//             println!("-- 1");
//             let s_cover_group_tag_block_list = get_tag_block_list(&result, "s_cover_group", None, None);
//             for s_cover_group_tag_block in s_cover_group_tag_block_list {
//                 println!("-- 2");
//                 let mut s_cover_group_tag_block_new = s_cover_group_tag_block.clone().replace(r#"<s_cover_group>"#, "").replace(r#"</s_cover_group>"#, "");
//                 let s_cover_rep_tag_block_list: Vec<String> = get_tag_block_list(&s_cover_group_tag_block_new, "s_cover_rep", None, None);
//                 for s_cover_rep_tag_block in s_cover_rep_tag_block_list {
//                     println!("-- 3");
//                     let mut s_cover_rep_tag_block_new = s_cover_rep_tag_block.clone().replace(r#"<s_cover_rep>"#, "").replace(r#"</s_cover_rep>"#, "");
//                     let s_cover_tag_block_list = get_tag_block_list(&s_cover_rep_tag_block_new, "s_cover", None, Some(true));
//                     let s_cover_with_attr_regex = Regex::new(r#"<s_cover\s+(.*?)>"#).unwrap();
//                     let s_cover_tag_block_option = s_cover_tag_block_list.iter().find(|s| {
//                         let tag_and_attrs_string = s_cover_with_attr_regex.find(s).unwrap().as_str();
//                         Regex::new(format!(r#"name=.{}."#, name).as_str()).unwrap().is_match(tag_and_attrs_string)
//                     });
//                     if let Some(s_cover_tag_block) = s_cover_tag_block_option {
//                         println!("-- 4 {:#?}", s_cover_with_attr_regex.is_match(&s_cover_tag_block));

//                         let mut s_cover_tag_block_new = s_cover_with_attr_regex.replace(&s_cover_tag_block, "").replace(r#"</s_cover>"#, "");
//                         println!("#@@@ s_cover_tag_block_new {:#?}", s_cover_tag_block_new);
//                         // let mut s_cover_tag_block_new = s_cover_tag_block.clone().replace(r#"<s_cover>"#, "").replace(r#"</s_cover>"#, "");
//                         s_cover_tag_block_new = s_cover_tag_block_new.replace(r#"[##_cover_title_##]"#, title.as_str());
//                         let s_cover_item_tag_block_list = get_tag_block_list(&s_cover_tag_block_new, "s_cover_item", None, None);
//                         for s_cover_item_tag_block in s_cover_item_tag_block_list {
//                             println!("-- 5");
//                             let mut s_cover_item_tag_block_new = String::new();
//                             let mut items: Vec<String> = vec![];
//                             for post in &posts {
//                                 println!("-- 6");
//                                 let mut s_cover_item_tag_block_template = s_cover_item_tag_block.clone().replace(r#"<s_cover_item>"#, "").replace(r#"</s_cover_item>"#, "");
//                                 let s_cover_item_article_info_tag_block_list = get_tag_block_list(&s_cover_item_tag_block_template, "s_cover_item_article_info", None, None);
//                                 for s_cover_item_article_info_tag_block in s_cover_item_article_info_tag_block_list {
//                                     println!("-- 7");
//                                     let mut s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block.clone().replace(r#"<s_cover_item_article_info>"#, "").replace(r#"</s_cover_item_article_info>"#, "");;
//                                     s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block_new.replace(r#"[##_cover_item_url_##]"#, format!(r#"/{}"#, post.post_id.clone().unwrap()).as_str());
//                                     s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block_new.replace(r#"[##_cover_item_title_##]"#, format!(r#"{}"#, post.title.clone().unwrap()).as_str());
//                                     s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block_new.replace(r#"[##_cover_item_date_##]"#, format!(r#"{}"#, NaiveDateTime::parse_from_str(post.created_at.clone().unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap().format("%Y-%m-%d %H:%M").to_string().as_str()).as_str());
//                                     s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block_new.replace(r#"[##_cover_item_summary_##]"#, format!(r#"{}"#, post.get_contents_summary().as_str()).as_str());
//                                     let s_cover_item_thumbnail_tag_block_list = get_tag_block_list(&s_cover_item_article_info_tag_block_new, "s_cover_item_thumbnail", None, None);
//                                     for s_cover_item_thumbnail_tag_block in s_cover_item_thumbnail_tag_block_list {
//                                         println!("-- 8");
//                                         let mut s_cover_item_thumbnail_tag_block_new = s_cover_item_thumbnail_tag_block.clone().replace(r#"<s_cover_item_thumbnail>"#, "").replace(r#"</s_cover_item_thumbnail>"#, "");
//                                         s_cover_item_thumbnail_tag_block_new = s_cover_item_thumbnail_tag_block_new.replace(r#"//i1.daumcdn.net/thumb/C148x148/?fname=[##_cover_item_thumbnail_##]"#, &post.thumbnail_img_url.clone().unwrap_or_else(|| "".to_string()));

//                                         s_cover_item_article_info_tag_block_new = s_cover_item_article_info_tag_block_new.replace(&s_cover_item_thumbnail_tag_block, &s_cover_item_thumbnail_tag_block_new);
//                                     }
//                                     s_cover_item_tag_block_template = s_cover_item_tag_block_template.replace(&s_cover_item_article_info_tag_block, &s_cover_item_article_info_tag_block_new);
//                                 }
//                                 items.push(s_cover_item_tag_block_template);
//                                 // s_cover_tag_block_new = s_cover_tag_block_new.replace(&s_cover_item_tag_block, &s_cover_item_tag_block_new);
//                             }
//                             s_cover_item_tag_block_new = items.join("");
//                             s_cover_tag_block_new = s_cover_tag_block_new.replace(&s_cover_item_tag_block, &s_cover_item_tag_block_new);
//                         }
//                         // println!("s_cover_tag_block_new ::: {:#?}", s_cover_tag_block_new);
//                         s_cover_rep_tag_block_new = s_cover_rep_tag_block_new.replace(s_cover_tag_block, s_cover_tag_block_new.as_str());
//                         println!("s_cover_rep_tag_block_new ::: {:#?}", s_cover_rep_tag_block_new);
//                     }
//                     s_cover_group_tag_block_new = s_cover_group_tag_block_new.replace(s_cover_rep_tag_block.as_str(), s_cover_rep_tag_block_new.as_str());    
//                 } 
//                 result = result.replace(s_cover_group_tag_block.as_str(), s_cover_group_tag_block_new.as_str());
//             }
//         }
//     } else {
//         // 홈 화면을 최신글로..

//     }

//     result
// }