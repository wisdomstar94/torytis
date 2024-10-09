use std::{ops::Deref, rc::Rc};
use chrono::NaiveDateTime;
use html_regex::{html_string_root_element_unwrap, select_from_html_string, select_from_html_string_one, Bucket, Bucket2, SelectOptions};

use crate::{common::{date_format, get_pagination_calculate}, structs::torytis_dev_config::{Post, PostType, TorytisDevConfig}};

use super::torytis_dev_config::{get_skin_variable_info_map, GuestbookSelectOption, PostSelectOption};

pub struct Replacer {
    root: Rc<Bucket>,
    config: TorytisDevConfig,
    s_article_index_rep_template: String,
    s_article_permalink_rep_template: String,
    s_notice_index_rep_template: String,
    s_notice_permalink_rep_template: String,
    s_article_protected_index_rep_template: String,
    s_article_protected_permalink_rep_template: String,
}

impl Replacer {
    pub fn new(html: &str) -> Self {
        let config = TorytisDevConfig::new();
        let mut result = html.to_owned();
        let mut skin_variable_info_map = get_skin_variable_info_map();

        let skin_setting_variables = config.get_skin_setting_variables();
        for (_, info) in skin_variable_info_map.drain() {
            let mut seted_value: Option<String> = None;
            if let Some(hashmap) = &skin_setting_variables {
                if let Some(sv) = hashmap.get(&info.var_name) {
                    seted_value = Some(sv.clone());
                }
            }
            let default: Option<String> = info.default;
            let mut this_var_value: Option<String> = None;
            if let Some(v) = seted_value {
                this_var_value = Some(v);
            } else if let Some(v) = default {
                this_var_value = Some(v);
            }

            let if_tag_name = format!("s_if_var_{}", info.var_name);
            let not_tag_name = format!("s_not_var_{}", info.var_name);
            if let Some(v) = this_var_value {
                result = result.replace(info.var_code_name.as_str(), v.as_str());
                let root = Bucket::new(&result);
                root
                    .select(SelectOptions { 
                        element_name: if_tag_name.as_str(),
                        attrs: None,
                        is_attrs_check_string_contain: true, 
                    })
                    .replacer(|_, matched_str_unwarp| {
                        matched_str_unwarp.unwrap()
                    })
                    .commit()
                ;
                root
                    .select(SelectOptions { 
                        element_name: not_tag_name.as_str(),
                        attrs: None,
                        is_attrs_check_string_contain: true, 
                    })
                    .replacer(|_, _| {
                        String::new()
                    })  
                    .commit()
                ;
                result = root.get_html();
            } else {
                let root = Bucket::new(&result);
                root
                    .select(SelectOptions { 
                        element_name: if_tag_name.as_str(),
                        attrs: None,
                        is_attrs_check_string_contain: true, 
                    })
                    .replacer(|_, _| {
                        String::new()
                    })
                    .commit()
                ;
                root
                    .select(SelectOptions { 
                        element_name: not_tag_name.as_str(),
                        attrs: None,
                        is_attrs_check_string_contain: true, 
                    })  
                    .replacer(|_, matched_str_unwarp| {
                        matched_str_unwarp.unwrap()
                    })
                    .commit()
                ;
                result = root.get_html();
            }
        }

        let mut s_article_index_rep_template: String = String::new();
        let mut s_article_permalink_rep_template: String = String::new();
        let mut s_notice_index_rep_template: String = String::new();
        let mut s_notice_permalink_rep_template: String = String::new();
        let mut s_article_protected_index_rep_template: String = String::new();
        let mut s_article_protected_permalink_rep_template: String = String::new();

        // s_article_rep
        let s_article_rep = select_from_html_string_one(&result, &SelectOptions {
            element_name: "s_article_rep",
            attrs: None,
            is_attrs_check_string_contain: true
        });
        if let Some(s_article_rep) = s_article_rep {
            let index_template = select_from_html_string_one(&s_article_rep, &SelectOptions {
                element_name: "s_index_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(index_template) = index_template {
                s_article_index_rep_template = html_string_root_element_unwrap(&index_template, "s_index_article_rep");
            }
            let permalink_template = select_from_html_string_one(&s_article_rep, &SelectOptions {
                element_name: "s_permalink_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(permalink_template) = permalink_template {
                s_article_permalink_rep_template = html_string_root_element_unwrap(&permalink_template, "s_permalink_article_rep");
            }
            result = result.replace(s_article_rep.as_str(), "<s_article_rep></s_article_rep>");
        }

        // s_notice_rep
        let s_notice_rep = select_from_html_string_one(&result, &SelectOptions {
            element_name: "s_notice_rep",
            attrs: None,
            is_attrs_check_string_contain: true
        });
        if let Some(s_notice_rep) = s_notice_rep {
            let index_template = select_from_html_string_one(&s_notice_rep, &SelectOptions {
                element_name: "s_index_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(index_template) = index_template {
                s_notice_index_rep_template = html_string_root_element_unwrap(&index_template, "s_index_article_rep");
                // println!("s_notice_index_rep_template {}", s_notice_index_rep_template);
            }
            let permalink_template = select_from_html_string_one(&s_notice_rep, &SelectOptions {
                element_name: "s_permalink_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(permalink_template) = permalink_template {
                // println!("notice permalink_template {}", permalink_template);
                s_notice_permalink_rep_template = html_string_root_element_unwrap(&permalink_template, "s_permalink_article_rep");
                // println!("s_notice_permalink_rep_template {}", s_notice_permalink_rep_template);
            }
            // let is_matched = result.matches(s_notice_rep.as_str()).count() > 0;
            // println!("@@@@@@@ s_notice_rep {}", s_notice_rep);
            // println!("@@@@@@@ is_matched {}", is_matched);
            result = result.replace(s_notice_rep.as_str(), "");
        }

        // s_article_protected
        let s_article_protected = select_from_html_string_one(&result, &SelectOptions {
            element_name: "s_article_protected",
            attrs: None,
            is_attrs_check_string_contain: true
        });
        if let Some(s_article_protected) = s_article_protected {
            let index_template = select_from_html_string_one(&s_article_protected, &SelectOptions {
                element_name: "s_index_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(index_template) = index_template {
                s_article_protected_index_rep_template = html_string_root_element_unwrap(&index_template, "s_index_article_rep");
            }
            let permalink_template = select_from_html_string_one(&s_article_protected, &SelectOptions {
                element_name: "s_permalink_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            }); 
            if let Some(permalink_template) = permalink_template {
                s_article_protected_permalink_rep_template = html_string_root_element_unwrap(&permalink_template, "s_permalink_article_rep");
            }
            result = result.replace(s_article_protected.as_str(), "");
        }

        Self {
            root: Bucket::new(&result),
            config,
            s_article_index_rep_template,
            s_article_permalink_rep_template,
            s_notice_index_rep_template,
            s_notice_permalink_rep_template,
            s_article_protected_index_rep_template,
            s_article_protected_permalink_rep_template,
        }
    }

    pub fn get_s_article_index_rep_template(&self) -> String {
        self.s_article_index_rep_template.clone()
    }
    pub fn get_s_article_permalink_rep_template(&self) -> String {
        self.s_article_permalink_rep_template.clone()
    }
    pub fn get_s_notice_index_rep_template(&self) -> String {
        self.s_notice_index_rep_template.clone()
    }
    pub fn get_s_notice_permalink_rep_template(&self) -> String {
        self.s_notice_permalink_rep_template.clone()
    }
    pub fn get_s_article_protected_index_rep_template(&self) -> String {
        self.s_article_protected_index_rep_template.clone()
    }
    pub fn get_s_article_protected_permalink_rep_template(&self) -> String {
        self.s_article_protected_permalink_rep_template.clone()
    }

    pub fn get_html(&self) -> String {
        self.root.get_html()
    }
}

impl Replacer {
    
}

impl Replacer {
    fn apply_common(&self, options: ApplyCommonOptions) {
        // let me = self;
        let root = Rc::clone(&self.root);
        let config = self.config.get_clone_rc();

        let category_list_html = Rc::new(config.get_category_list_html());
        let count_total = Rc::new(config.get_visitor().unwrap().count_total.unwrap().to_string());
        let count_today = Rc::new(config.get_visitor().unwrap().count_today.unwrap().to_string());
        let count_yesterday = Rc::new(config.get_visitor().unwrap().count_yesterday.unwrap().to_string());
        let recent_comment_list = Rc::new(config.get_recent_comment_list().clone().unwrap());
        let recent_post_list = Rc::new(config.get_recent_post_list().clone().unwrap());
        let recent_notice_list = Rc::new(config.get_recent_notice_list().clone().unwrap());

        let options_search = options.search;
        let body_id = options.body_id;

        let total_posts = Rc::new(self.config.get_posts(None));

        root
            .html_str_replace(|html| {
                html.replace(r#"[##_body_id_##]"#, &body_id)
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_revenue_list_upper_##]"#, "")
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_revenue_list_lower_##]"#, "")
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_title_##]"#, &config.get_blog_title().unwrap())
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_blogger_##]"#, &config.get_blog_profile_name().unwrap())
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_image_##]"#, &config.get_blog_profile_img_url().unwrap())
            })
            .html_str_replace(|html| {
                html.replace(r#"[##_desc_##]"#, &config.get_blog_description().unwrap())
            })
            .html_str_replace(|html| {
                html.replace(r#"<link href="./style.css" type="text/css" rel="stylesheet" />"#, r#"
                    <link href="/tistorycdn/content.css" type="text/css" rel="stylesheet" />
                    <link href="/tistorycdn/postBtn.css" type="text/css" rel="stylesheet" />
                    <link href="/tistorycdn/another_category.css" type="text/css" rel="stylesheet" />
                    <link href="/tistorycdn/comment.css" type="text/css" rel="stylesheet" />
                    <link href="/virtualcdn/style.css" type="text/css" rel="stylesheet" />
                "#)
            })
            .html_str_replace(|html| {
                html.replace(r#"./style.css"#, r#"/virtualcdn/style.css"#)
            })
            .html_str_replace(|html| {
                html.replace(r#"./images/"#, r#"/virtualcdn/images/"#).replace(r#"'/images/"#, r#"'/virtualcdn/images/"#).replace(r#""/images/"#, r#""/virtualcdn/images/"#)
            })
            .select(SelectOptions {
                element_name: "s_sidebar",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(|_, unwrap_matched_str| {
                let s_sidebar_inner_matched_str = unwrap_matched_str.unwrap();

                let htmls = select_from_html_string(&s_sidebar_inner_matched_str, &SelectOptions { element_name: "s_sidebar_element", attrs: None, is_attrs_check_string_contain: false });
                
                htmls.join("")
            })
            .commit()
        ;
            
        root
            .select(SelectOptions {
                element_name: "s_sidebar_element",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, unwrap_matched_str| {
                // let mut result = unwrap_matched_str.unwrap();
                let s_sidebar_element_unwrap = Bucket::new(&unwrap_matched_str.unwrap());

                let recent_comment_list = recent_comment_list.clone();
                let recent_notice_list = recent_notice_list.clone();

                // 카테고리 리스트 치환
                s_sidebar_element_unwrap
                    .html_str_replace(|s| {
                        s.replace(r#"[##_category_list_##]"#, &category_list_html)
                    })
                ;

                // 방문자수 치환
                s_sidebar_element_unwrap
                    .html_str_replace(|s| {
                        s.replace(r#"[##_count_total_##]"#, &count_total.as_str())
                    })
                    .html_str_replace(|s| {
                        s.replace(r#"[##_count_today_##]"#, &count_today.as_str())
                    })
                    .html_str_replace(|s| {
                        s.replace(r#"[##_count_yesterday_##]"#, &count_yesterday.as_str())
                    })
                ;

                let total_posts = Rc::clone(&total_posts);

                // 최근 댓글
                s_sidebar_element_unwrap
                    .select(SelectOptions {
                        element_name: "s_rctrp_rep",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        let recent_comment_list = recent_comment_list.clone();
                        let html_template: String = matched_str_unwrap.unwrap();
                        let mut li_vec: Vec<String> = Vec::new();
                        // let k = recent_comment_list.clone();
                        for item in recent_comment_list.deref() {
                            let bucket = Bucket::new(&html_template);
                            let name = item.name.as_ref().unwrap().to_owned();
                            let time = item.datetime.as_ref().unwrap().to_owned();
                            let desc = item.content.as_ref().unwrap().to_owned();
                            let total_posts = Rc::clone(&total_posts);
                            let target_post_id = TorytisDevConfig::get_post_id_from_comment_id(total_posts.deref().clone(), item.comment_id.clone().unwrap().as_str()).unwrap();
                            let post_url = format!("/{}#{}", target_post_id, item.comment_id.clone().unwrap().as_str());
                            bucket
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctrp_rep_name_##]"#, &name)
                                })
                                .html_str_replace(|s| {
                                    let time = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap().format("%m.%d").to_string();
                                    s.replace(r#"[##_rctrp_rep_time_##]"#, &time)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctrp_rep_link_##]"#, post_url.as_str())
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctrp_rep_desc_##]"#, &desc)
                                })
                            ;
                            li_vec.push(bucket.get_html());
                        }
                        li_vec.join("")
                    })
                    .commit()
                ;

                // 최근 게시글
                let recent_post_list_copy = recent_post_list.clone();
                s_sidebar_element_unwrap
                    .select(SelectOptions {
                        element_name: "s_rctps_rep",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        let k = &recent_post_list_copy;
                        let list = Rc::clone(k);
                        let html_template: String = matched_str_unwrap.unwrap();
                        let mut li_vec: Vec<String> = Vec::new();
                        
                        for item in list.deref() {
                            let bucket = Bucket::new(&html_template);
                            let link = format!("/{}", item.post_id.as_ref().unwrap()).to_owned();
                            let title = item.title.as_ref().unwrap().to_owned();
                            let rp_cnt = item.comment_list.as_ref().unwrap().len().to_string();
                            let author = item.author.as_ref().unwrap().to_owned();
                            let created_at = item.created_at.as_ref().unwrap().to_owned();
                            let category_name = item.category_name.as_ref().unwrap().to_owned();
                            let category_link = format!("/category/{}", category_name).to_owned();
                            let thumbnail_img_url = item.thumbnail_img_url.as_ref().unwrap().to_owned();
                            bucket
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_link_##]"#, &link)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_title_##]"#, &title)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_rp_cnt_##]"#, &rp_cnt)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_author_##]"#, &author)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_category_##]"#, &category_name)
                                })
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctps_rep_category_link_##]"#, &category_link)
                                })
                                .html_str_replace(|s| {
                                    let time = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d %H:%M").to_string();
                                    s.replace(r#"[##_rctps_rep_date_##]"#, &time)
                                })
                                .html_str_replace(|s| {
                                    let time = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d").to_string();
                                    s.replace(r#"[##_rctps_rep_simple_date_##]"#, &time)
                                })
                                .select(SelectOptions {
                                    element_name: "s_rctps_rep_thumbnail",
                                    attrs: None,
                                    is_attrs_check_string_contain: true,
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    if thumbnail_img_url == String::from("") {
                                        return String::from("");
                                    }

                                    let mut result = matched_str_unwrap.unwrap();
                                    result = result.replace(r#"[##_rctps_rep_thumbnail_##]"#, &thumbnail_img_url.clone());
                                    result
                                })
                                .commit()
                            ;
                            li_vec.push(bucket.get_html());
                        }
                        li_vec.join("")
                    })
                    .commit()
                ;

                // 검색
                s_sidebar_element_unwrap
                    .html_str_replace(|s| {
                        s.replace(r#"[##_search_name_##]"#, "search")
                    })
                    .html_str_replace(|s| {
                        s.replace(r#"[##_search_text_##]"#, &options_search)
                    })
                    .html_str_replace(|s| {
                        s.replace(r#"[##_search_onclick_submit_##]"#, r#"
                            try {
                                window.location.href = '/search' + '/' + encodeURI(document.getElementsByName('search')[0].value);
                                document.getElementsByName('search')[0].value = '';
                                return false;
                            } catch (e) {
                                
                            } 
                        "#)
                    })
                ;

                // 최근 공지사항
                s_sidebar_element_unwrap
                    .select(SelectOptions {
                        element_name: "s_rct_notice",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(|_, matched_str_unwrap| {
                        matched_str_unwrap.unwrap()
                    })
                    .select(SelectOptions {
                        element_name: "s_rct_notice_rep",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        let recent_notice_list = recent_notice_list.clone();
                        let html_template = matched_str_unwrap.unwrap();
                        let mut list_vec: Vec<String> = Vec::new();
                        let iter = recent_notice_list.iter();
                        for item in iter {
                            let bucket = Bucket::new(&html_template);
                            let post_id = item.post_id.clone().unwrap();
                            let notice_title = item.title.clone().unwrap();
                            bucket
                                .html_str_replace(move |s| {
                                    s.replace(r#"[##_notice_rep_link_##]"#, format!("/notice/{}", post_id).as_str())
                                })
                                .html_str_replace(move |s| {
                                    s.replace(r#"[##_notice_rep_title_##]"#, &notice_title)
                                })
                            ;
                            list_vec.push(bucket.get_html());
                        }
                        list_vec.join("")
                    })
                    .commit()
                ;

                s_sidebar_element_unwrap.get_html()
            })
            
            .commit()
        ;
    }

    fn apply_home_cover(&self, option: ApplyHomeCoverOptions) {
        let is_hide = option.is_hide;

        let root = Rc::clone(&self.root);
        // let xml_cover_items = Rc::new(self.config.get_xml_cover_items());
        let skin_home_cover_setting_info = Rc::new(self.config.get_skin_home_cover());

        let posts = Rc::new(self.config.get_posts(None));

        root
            .select(SelectOptions {
                element_name: "s_cover_group",
                attrs: None,
                is_attrs_check_string_contain: true
            })
            .replacer(move |_, matched_str_unwrap| {
                if is_hide {
                    return String::new();
                }

                matched_str_unwrap.unwrap()
            })
            .select(SelectOptions {
                element_name: "s_cover_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            })
            .replacer(move |_, matched_str_unwrap| {
                let html_stores = matched_str_unwrap.unwrap();
                let mut list_vec: Vec<String> = Vec::new();
                
                let skin_home_cover_setting_info = Rc::clone(&skin_home_cover_setting_info);
                if let Some(ss) = skin_home_cover_setting_info.deref() {
                    let cover_items = ss.cover_items.clone().unwrap();
                    for cover_item in cover_items {
                        let template_html_wrap = select_from_html_string_one(&html_stores, &SelectOptions {
                            element_name: "s_cover",
                            attrs: Some(vec![("name", &cover_item.cover_name.unwrap())]),
                            is_attrs_check_string_contain: false
                        });
                        if let Some(template_html) = template_html_wrap {
                            let result = html_string_root_element_unwrap(&template_html, "s_cover");
                            let mini_root = Bucket::new(&result);
                            mini_root
                                .html_str_replace(|html| {
                                    html.replace(r#"[##_cover_title_##]"#, &cover_item.cover_title.clone().unwrap())
                                })
                            ;

                            let target_posts = posts.deref().clone().unwrap().iter().filter(|x| {
                                x.category_name == cover_item.cover_category_name
                            }).map(|x| x.clone()).collect::<Vec<Post>>();

                            // s_cover_item
                            mini_root
                                .select(SelectOptions {
                                    element_name: "s_cover_item",
                                    attrs: None,
                                    is_attrs_check_string_contain: true
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    let template = matched_str_unwrap.unwrap();
                                    let mut li_vec: Vec<String> = Vec::new();
                                    for item in &target_posts {
                                        let mini_root2 = Bucket::new(&template);
                                        let post = Rc::new(item.clone());

                                        // s_cover_item_article_info
                                        mini_root2
                                            .select(SelectOptions {
                                                element_name: "s_cover_item_article_info",
                                                attrs: None,
                                                is_attrs_check_string_contain: true
                                            })
                                            .replacer(move |_, matched_str_unwrap| {
                                                let r = matched_str_unwrap.unwrap();
                                                let mini_root3 = Bucket::new(&r);
                                                let post = Rc::clone(&post);

                                                mini_root3
                                                    .html_str_replace(|h| {
                                                        h.replace(r#"[##_cover_item_url_##]"#, format!(r#"/{}"#, post.post_id.clone().unwrap().as_str()).as_str())
                                                    })
                                                    .html_str_replace(|h| {
                                                        h.replace(r#"[##_cover_item_title_##]"#, format!(r#"/{}"#, post.title.clone().unwrap().as_str()).as_str())
                                                    })
                                                    .html_str_replace(|h| {
                                                        h.replace(r#"[##_cover_item_date_##]"#, date_format(post.created_at.clone().unwrap().as_str(), "%Y-%m-%d %H:%M").as_str())
                                                    })
                                                    .html_str_replace(|h| {
                                                        h.replace(r#"[##_cover_item_summary_##]"#, format!(r#"/{}"#, post.get_contents_summary().clone().as_str()).as_str())
                                                    })
                                                ;

                                                mini_root3
                                                    .select(SelectOptions {
                                                        element_name: "s_cover_item_thumbnail",
                                                        attrs: None,
                                                        is_attrs_check_string_contain: true
                                                    })
                                                    .replacer(move |_, matched_str_unwrap| {
                                                        if post.thumbnail_img_url.is_none() {
                                                            return String::new();
                                                        }
                                                        let thumbnail_img_url = post.thumbnail_img_url.clone().unwrap();
                                                        let mut result = matched_str_unwrap.unwrap();
                                                        result = result.replace(r#"//i1.daumcdn.net/thumb/C148x148/?fname=[##_cover_item_thumbnail_##]"#, &thumbnail_img_url);
                                                        result = result.replace(r#"[##_cover_item_thumbnail_##]"#, &thumbnail_img_url);
                                                        result
                                                    })
                                                    .commit()
                                                ;

                                                mini_root3.get_html()
                                            })
                                            .commit()
                                        ;

                                        li_vec.push(mini_root2.get_html());
                                    }
                                    li_vec.join("")
                                })
                                .commit()
                            ;

                            // s_cover_url

                            list_vec.push(mini_root.get_html());
                        }
                    }
                }

                list_vec.join("")
            })
            .commit()
        ;
    }

    fn apply_index_list(&self, option: ApplyIndexListOptions) {
        let is_hide = option.is_hide;
        let root = Rc::clone(&self.root);
        // let mut post_select_option: Option<PostSelectOption> = None;
        // if let Some(v) = option.post_select_option {
        //     post_select_option = Some(v);
        // }
        let post_list = self.config.get_posts(option.post_select_option);
        // println!(">>> post_list: {:#?}", post_list);
        let normal_index_rep_template = self.get_s_article_index_rep_template();
        let notice_index_rep_template = self.get_s_notice_index_rep_template();
        let protected_index_rep_template = self.get_s_article_protected_index_rep_template();
        // println!("skin_variable_info_map {:#?}", skin_variable_info_map);
        let config = Rc::new(self.config.clone());

        fn common(root: &Bucket, item: Post, config: &Rc<TorytisDevConfig>) {
            let post_type = item.post_type.clone();
            let thumbnail_img_url1 = item.thumbnail_img_url.as_ref().unwrap().clone();
            let thumbnail_img_url2 = item.thumbnail_img_url.as_ref().unwrap().clone();
            let comment_list = item.comment_list.as_ref().unwrap().clone();
            let category_name = item.category_name.as_ref().unwrap().clone();
            let title = item.title.as_ref().unwrap().clone();
            let author = item.author.as_ref().unwrap().clone();
            let datetime = item.created_at.as_ref().unwrap().clone();
            let datetime_split: Vec<&str> = datetime.split(" ").collect();
            let date = datetime_split.get(0).unwrap();
            let date_split: Vec<&str> = date.split("-").collect();
            let date_year = date_split.get(0).unwrap().to_string();
            let date_month = date_split.get(1).unwrap().to_string();
            let date_date = date_split.get(2).unwrap().to_string();
            let time = datetime_split.get(1).unwrap();
            let time_split: Vec<&str> = time.split(":").collect();
            let time_hour = time_split.get(0).unwrap().to_string();
            let time_minute = time_split.get(1).unwrap().to_string();
            let time_second = time_split.get(2).unwrap().to_string();
            let content_summary = &item.get_contents_summary();
            let is_guest = config.get_is_guest();
            let is_private = item.is_private;

            root
                .html_str_replace(|html| {
                    let mut link = format!("/{}", &item.post_id.clone().unwrap());
                    if let Some(v) = &post_type {
                        if v.is_equal(&PostType::Notice) {
                            link = format!("/notice/{}", &item.post_id.clone().unwrap());
                        }
                    }
                    html.replace(r#"[##_article_rep_link_##]"#, link.as_str())
                })
            ;
            root
                .select(SelectOptions {
                    element_name: "s_article_rep_thumbnail",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    if thumbnail_img_url1 == String::from("") {
                        return String::from("");
                    }

                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_article_rep_thumbnail_url_##]"#, &thumbnail_img_url1.clone());
                    result
                })
                .commit()
            ;
            root
                .select(SelectOptions {
                    element_name: "s_rp_count",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_article_rep_rp_cnt_##]"#, &comment_list.clone().len().to_string());
                    result
                })
                .commit()
            ;
            root
                .select(SelectOptions {
                    element_name: "s_notice_rep_thumbnail",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    if thumbnail_img_url2 == String::from("") {
                        return String::from("");
                    }

                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_article_rep_thumbnail_url_##]"#, &thumbnail_img_url2.clone());
                    result
                })
                .commit()
            ;        
            root
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_category_##]"#, &category_name.replace("///", "/"))
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_title_##]"#, &title)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_author_##]"#, &author)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_year_##]"#, &date_year)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_month_##]"#, &date_month)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_day_##]"#, &date_date)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_hour_##]"#, &time_hour)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_minute_##]"#, &time_minute)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_second_##]"#, &time_second)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_summary_##]"#, &content_summary)
                })
                .html_str_replace(|html| {
                    let time = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d %H:%M").to_string();
                    html.replace(r#"[##_article_rep_date_##]"#, &time)
                })
                .html_str_replace(|html| {
                    let time = NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d").to_string();
                    html.replace(r#"[##_article_rep_simple_date_##]"#, &time)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_title_##]"#, &title)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_author_##]"#, &author)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_year_##]"#, &date_year)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_month_##]"#, &date_month)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_day_##]"#, &date_date)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_hour_##]"#, &time_hour)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_minute_##]"#, &time_minute)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_date_second_##]"#, &time_second)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_notice_rep_summary_##]"#, &content_summary)
                })
            ;

            root
                .select(SelectOptions {
                    element_name: "s_ad_div",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    if let Some(vv) = is_guest {
                        if vv {
                            return String::new();
                        }
                    }

                    let is_private = is_private.unwrap();
                    let status_string: &str = if post_type.clone().unwrap().is_equal(&PostType::Protected) {
                        "보호"
                    } else if is_private {
                        "비공개"
                    } else {
                        "공개"
                    };

                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_s_ad_s1_label_##]"#, status_string);
                    result
                })
                .commit()
            ;
        }

        root
            .select(SelectOptions {
                element_name: "s_article_rep",
                attrs: None,
                is_attrs_check_string_contain: true
            })
            .replacer(move |_, _| {
                // println!("s_article_rep.replacer called!!");
                // let a = matched_str_unwrap.unwrap();
                // a
                if is_hide {
                    return String::new();
                }

                let mut list_vec: Vec<String> = Vec::new();
                if let Some(post_list) = &post_list {
                    for item in post_list {
                        match item.post_type.clone().unwrap() {
                            // 일반 글
                            super::torytis_dev_config::PostType::Normal => {
                                let root = Bucket::new(&normal_index_rep_template);
                                common(&root, item.clone(), &config);
                                list_vec.push(root.get_html());
                            },
                            // 공지사항 글
                            super::torytis_dev_config::PostType::Notice => {
                                let root = Bucket::new(&notice_index_rep_template);
                                common(&root, item.clone(), &config);
                                list_vec.push(root.get_html());
                            },
                            // 암호로 보호된 글
                            super::torytis_dev_config::PostType::Protected => {
                                let root = Bucket::new(&protected_index_rep_template);
                                common(&root, item.clone(), &config);
                                list_vec.push(root.get_html());
                            },
                        }
                    }
                }
                // println!("list_vec: {:#?}", list_vec);
                list_vec.join("")
            })
            .commit()
        ;
    }

    fn apply_guest_book(&self, option: ApplyGuestBookOptions) {
        let is_hide = option.is_hide;
        let root = Rc::clone(&self.root);
        let guestbook_list = Rc::new(self.config.get_guestbooks(option.guestbook_select_option));
        // let guestbook_select_option = Rc::new(option.guestbook_select_option);
        let is_guest = Rc::new(self.config.get_is_guest());

        root    
            .select(SelectOptions {
                element_name: "s_guest",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwrap| {
                if is_hide {
                    return String::new();
                }

                let mini_root = Bucket::new(&matched_str_unwrap.unwrap());
                let guestbook_list = Rc::clone(&guestbook_list);
                let is_guest = Rc::clone(&is_guest);

                // s_guest_container
                mini_root
                    .select(SelectOptions {
                        element_name: "s_guest_container",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        matched_str_unwrap.unwrap()
                    })
                    .select(SelectOptions {
                        element_name: "s_guest_rep",
                        attrs: None,
                        is_attrs_check_string_contain: true,
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        let guestbook_list = Rc::clone(&guestbook_list);
                        let html_template = matched_str_unwrap.unwrap();
                        let mut list_vec: Vec<String> = Vec::new();
                        for item in guestbook_list.iter() {
                            let child_guestbook_list = Rc::new(item.guestbook_list.clone());
                            let child_guestbook_list2 = Rc::new(item.guestbook_list.clone());
                            let item_bucket = Bucket::new(&html_template);
                            item_bucket
                                .html_str_replace(|html| {
                                    html.replacen(r#"[##_guest_rep_id_##]"#, &item.guest_rep_id.clone().unwrap(), 1)
                                })
                                .html_str_replace(|html| {
                                    html.replacen(r#"[##_guest_rep_logo_##]"#, &item.guest_rep_logo.clone().unwrap(), 1)
                                })
                                .html_str_replace(|html| {
                                    html.replacen(r#"[##_guest_rep_name_##]"#, &item.name.clone().unwrap(), 1)
                                })
                                .html_str_replace(|html| {
                                    html.replacen(r#"[##_guest_rep_date_##]"#, date_format(&item.created_at.clone().unwrap(), "%Y-%m-%d %H:%M").as_str(), 1)
                                })
                                .html_str_replace(|html| {
                                    html.replacen(r#"[##_guest_rep_desc_##]"#, &item.content.clone().unwrap(), 1)
                                })
                                .html_str_replace(|html| {
                                    html.replace(r#"[##_guest_rep_onclick_reply_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#)
                                })
                                .html_str_replace(|html| {
                                    html.replace(r#"[##_guest_rep_onclick_delete_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#)
                                })
                                .select(SelectOptions {
                                    element_name: "s_guest_reply_container",
                                    attrs: None,
                                    is_attrs_check_string_contain: true,
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    if let None = child_guestbook_list.deref() {
                                        return String::new();
                                    }
                                    matched_str_unwrap.unwrap()
                                })
                                .select(SelectOptions {
                                    element_name: "s_guest_reply_rep",
                                    attrs: None,
                                    is_attrs_check_string_contain: true,
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    let html_template = matched_str_unwrap.unwrap();
                                    let mut li_vec: Vec<String> = Vec::new();
                                    if let Some(k) = child_guestbook_list2.deref() {
                                        for item in k {
                                            let mut result = html_template.clone();
                                            result = result.replace(r#"[##_guest_rep_id_##]"#, &item.guest_rep_id.clone().unwrap());
                                            result = result.replace(r#"[##_guest_rep_logo_##]"#, &item.guest_rep_logo.clone().unwrap());
                                            result = result.replace(r#"[##_guest_rep_name_##]"#, &item.name.clone().unwrap());
                                            result = result.replace(r#"[##_guest_rep_date_##]"#, date_format(&item.created_at.clone().unwrap(), "%Y-%m-%d %H:%M").as_str());
                                            result = result.replace(r#"[##_guest_rep_desc_##]"#, &item.content.clone().unwrap());
                                            li_vec.push(result);
                                        }
                                    }
                                    li_vec.join("") 
                                })
                                .commit()
                            ;
                            list_vec.push(item_bucket.get_html());
                        }
                        list_vec.join("")
                    })
                    .commit()
                ;

                // s_guest_input_form
                mini_root
                    .select(SelectOptions { 
                        element_name: "s_guest_input_form", 
                        attrs: None, 
                        is_attrs_check_string_contain: true, 
                    })
                    .replacer(move |_, matched_str_unwrap| {
                        let is_guest = Rc::clone(&is_guest);
                        let mut result = matched_str_unwrap.unwrap();
                        result = result.replace(r#"[##_guest_input_comment_##]"#, "comment");
                        result = result.replace(r#"[##_guest_onclick_submit_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);

                        let mini_root = Bucket::new(&result);

                        // s_guest_form
                        mini_root
                            .select(SelectOptions {
                                element_name: "s_guest_form",
                                attrs: None,
                                is_attrs_check_string_contain: true,
                            })
                            .replacer(move |_, matched_str_unwrap| {
                                let is_guest = Rc::clone(&is_guest);
                                if let Some(v) = is_guest.deref() {
                                    if v != &true {
                                        return String::new();
                                    }
                                }

                                let mut result = matched_str_unwrap.unwrap();
                                result = result.replace(r#"[##_guest_input_name_##]"#, r#"name"#);
                                result = result.replace(r#"[##_guest_name_##]"#, r#""#);
                                result = result.replace(r#"[##_guest_input_password_##]"#, r#"password"#);
                                result = result.replace(r#"[##_guest_password_##]"#, r#""#);
                                result
                            })
                            .commit()
                        ;

                        // s_rp_member
                        mini_root
                            .select(SelectOptions {
                                element_name: "s_rp_member",
                                attrs: None,
                                is_attrs_check_string_contain: true,
                            })
                            .replacer(move |_, matched_str_unwrap| {
                                let mut result = matched_str_unwrap.unwrap();
                                result = result.replace(r#"[##_rp_input_is_secret_##]"#, r#"secret"#);
                                result
                            })
                            .commit()
                        ;

                        mini_root.get_html()
                    })
                    .commit()
                ;
                
                let resut_result = format!(r#"<div id="entry0Comment">{}</div>"#, mini_root.get_html());
                resut_result
            })
            .commit()
        ;
    }

    fn apply_tag_list(&self, option: ApplyTagListOptions) {
        let is_hide = option.is_hide;
        let root = Rc::clone(&self.root);
        let tag_unique_list: Vec<String> = self.config.get_tag_unique_list();

        root    
            .select(SelectOptions {
                element_name: "s_tag",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwarp| {
                if is_hide {
                    return String::new();
                }
                matched_str_unwarp.unwrap()
            })
            .select(SelectOptions {
                element_name: "s_tag_rep",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwarp| {
                let html_template = matched_str_unwarp.unwrap();

                let mut list_vec: Vec<String> = Vec::new();
                for item in &tag_unique_list {
                    let mut result = html_template.clone();
                    result = result.replace(r#"[##_tag_link_##]"#, format!(r#"/tag/{}"#, item.as_str()).as_str());
                    result = result.replace(r#"[##_tag_name_##]"#, format!(r#"{}"#, item.as_str()).as_str());
                    list_vec.push(result);
                }

                list_vec.join("")
            })
            .commit()
        ;
    }

    fn apply_pagination(&self, option: ApplyPaginationOptions) {
        let is_hide = option.is_hide;
        let root = Rc::clone(&self.root);
        let pagination_info = Rc::new(option.pagination_info);
        let pagination_info1 = Rc::clone(&pagination_info);
        let pagination_info2 = Rc::clone(&pagination_info);

        root    
            .select(SelectOptions {
                element_name: "s_paging",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwrap| {
                if is_hide {
                    return String::new();
                }

                let mut result = matched_str_unwrap.unwrap();
                if let Some(v) = pagination_info1.deref() {
                    let cal = get_pagination_calculate(v.total_count, v.page, v.size);

                    let mut prev_button_page = v.page - 1;
                    if prev_button_page <= 0 {
                        prev_button_page = 1;
                    }
                    result = result.replace(r#"[##_prev_page_##]"#, format!(r#" href="{}?page={}" "#, v.base_url, prev_button_page).as_str());

                    let mut next_button_page = v.page + 1;
                    if next_button_page > cal.max_page_num {
                        next_button_page = cal.max_page_num.clone();
                    }
                    result = result.replace(r#"[##_next_page_##]"#, format!(r#" href="{}?page={}" "#, v.base_url, next_button_page).as_str());
                }

                result
            })
            .select(SelectOptions { 
                element_name: "s_paging_rep",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwrap| {
                let mut list_vec: Vec<String> = Vec::new();
                let html_template = matched_str_unwrap.unwrap();
                if let Some(v) = pagination_info2.deref() {
                    // let mut result = html_template.clone();
                    let cal = get_pagination_calculate(v.total_count, v.page, v.size);

                    if cal.best_left_num.is_some() {
                        let best_left_num = cal.best_left_num.unwrap();
                        list_vec.push(html_template.clone()
                            .replace(r#"[##_paging_rep_link_##]"#, format!(r#" href="{}?page={}" "#, v.base_url, best_left_num).as_str())
                            .replace(r#"[##_paging_rep_link_num_##]"#, format!(r#"<span>{}</span>"#, best_left_num.to_string().as_str()).as_str())
                        );
                        list_vec.push(html_template.clone()
                            .replace(r#"[##_paging_rep_link_##]"#, format!(r#""#).as_str())
                            .replace(r#"[##_paging_rep_link_num_##]"#, format!(r#"<span>···</span>"#).as_str())
                        );
                    }

                    for item in cal.center_page_num_list {
                        let mut span_class = String::new();
                        if item == v.page {
                            span_class = String::from("selected");
                        }
                        list_vec.push(html_template.clone()
                            .replace(r#"[##_paging_rep_link_##]"#, format!(r#" href="{}?page={}" "#, v.base_url, item.to_string().as_str()).as_str())
                            .replace(r#"[##_paging_rep_link_num_##]"#, format!(r#"<span class="{}">{}</span>"#, span_class, item.to_string().as_str()).as_str())
                        );      
                    }

                    if cal.best_right_num.is_some() {
                        let best_right_num = cal.best_right_num.unwrap();
                        list_vec.push(html_template.clone()
                            .replace(r#"[##_paging_rep_link_##]"#, format!(r#""#).as_str())
                            .replace(r#"[##_paging_rep_link_num_##]"#, format!(r#"<span>···</span>"#).as_str())
                        );
                        list_vec.push(html_template.clone()
                            .replace(r#"[##_paging_rep_link_##]"#, format!(r#" href="{}?page={}" "#, v.base_url, best_right_num).as_str())
                            .replace(r#"[##_paging_rep_link_num_##]"#, format!(r#"<span>{}</span>"#, best_right_num.to_string().as_str()).as_str())
                        );
                    }
                }
                list_vec.join("")
            })
            .commit()
        ;
    }

    fn apply_post_permalink(&self, option: ApplyPostPermalink) {
        let root = Rc::clone(&self.root);
        let config = Rc::new(self.config.clone());
        // let is_guest = Rc::new(self.config.get_is_guest());
        // let s_article_index_rep_template = Rc::new(self.get_s_article_index_rep_template());
        let s_article_permalink_rep_template = Rc::new(self.get_s_article_permalink_rep_template());
        // let s_notice_index_rep_template = Rc::new(self.get_s_notice_index_rep_template());
        let s_notice_permalink_rep_template = Rc::new(self.get_s_notice_permalink_rep_template());
        // let s_article_protected_index_rep_template = Rc::new(self.get_s_article_protected_index_rep_template());
        let s_article_protected_permalink_rep_template = Rc::new(self.get_s_article_protected_permalink_rep_template());

        fn common(target: &Rc<Bucket>, config: &Rc<TorytisDevConfig>, post: &Rc<Post>) {
            let is_guest = Rc::new(config.get_is_guest());
            let is_guest2 = Rc::clone(&is_guest);
            let is_private = Rc::new(post.is_private);
            let post_id  = Rc::new(post.post_id.clone().unwrap().to_string());
            let post_title = Rc::new(post.title.clone());
            let post_type = post.post_type.clone();
            let post_category_name = Rc::new(post.category_name.clone());
            let tag_list = Rc::new(post.tag_list.clone());
            let post_created_at = Rc::new(post.created_at.clone());
            let created_at = post_created_at.deref().as_ref().unwrap();
            let datetime_split: Vec<&str> = created_at.split(" ").collect();
            let date = datetime_split.get(0).unwrap();
            let date_split: Vec<&str> = date.split("-").collect();
            let date_year = Rc::new(date_split.get(0).unwrap().to_string());
            let date_month = Rc::new(date_split.get(1).unwrap().to_string());
            let date_date = Rc::new(date_split.get(2).unwrap().to_string());
            let time = datetime_split.get(1).unwrap();
            let time_split: Vec<&str> = time.split(":").collect();
            let time_hour = Rc::new(time_split.get(0).unwrap().to_string());
            // let time_minute = Rc::new(time_split.get(1).unwrap().to_string());
            let time_second = Rc::new(time_split.get(2).unwrap().to_string());
            let contents = Rc::new(post.get_contents());
            let category_name = Rc::new(post.category_name.clone());
            let post_next_and_prev = Rc::new(config.get_next_and_prev_post(post.post_id.clone()));
            let post_next_and_prev2 = Rc::clone(&post_next_and_prev);
            let comment_list = Rc::new(post.comment_list.clone());
            let comment_list2 = Rc::clone(&comment_list);
            // let blog_title = Rc::new(config.get_blog_title());
            // let blog_description = Rc::new(config.get_blog_description());
            let config2 = Rc::clone(config);

            // s_ad_div
            target
                .select(SelectOptions {
                    element_name: "s_ad_div",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    if let Some(v) = is_guest.deref() {
                        if v == &true {
                            return String::new();
                        }
                    }
                    matched_str_unwrap.unwrap()
                })
                .commit()
            ;

            target
                .html_str_replace(|html| {
                    let is_private = is_private.deref().unwrap();
                    let status_string: &str = if post_type.clone().unwrap().is_equal(&PostType::Protected) {
                        "보호"
                    } else if is_private {
                        "비공개"
                    } else {
                        "공개"
                    };
                    html.replace(r#"[##_s_ad_s1_label_##]"#, status_string)
                }) 
                .html_str_replace(|html| {
                    let title = post_title.deref().as_ref().unwrap();
                    html.replace(r#"[##_article_rep_title_##]"#, title.as_str())
                })
                .html_str_replace(|html| {
                    let category_name = post_category_name.deref().as_ref().unwrap();
                    let binding = category_name.replace("///", "/");
                    let c = binding.as_str();
                    html.replace(r#"[##_article_rep_category_link_##]"#, format!(r#"/category/{}"#, c).as_str())
                })
                .html_str_replace(|html| {
                    let category_name = post_category_name.deref().as_ref().unwrap();
                    let binding = category_name.replace("///", "/");
                    let c = binding.as_str();
                    html.replace(r#"[##_article_rep_category_##]"#, c)
                })
            ;

            // s_tag_label
            target
                .select(SelectOptions {
                    element_name: "s_tag_label",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    if let None = tag_list.deref().as_ref() {
                        return String::new();
                    }
                    let tag_list = tag_list.deref().as_ref().unwrap();

                    let mut list_vec: Vec<String> = Vec::new();
                    for item in tag_list {
                        let html_str = format!(r#"<a href="/tag/{}" rel="tag">{}</a>"#, item, item);
                        list_vec.push(html_str);
                    }
                    let list_html = list_vec.join(", ");

                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_tag_label_rep_##]"#, &list_html);
                    result
                })
                .commit()
            ;

            target
                .html_str_replace(|html| {
                    let time = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d %H:%M").to_string();
                    html.replace(r#"[##_article_rep_date_##]"#, &time)
                })
                .html_str_replace(|html| {
                    let time = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S").unwrap().format("%Y.%m.%d").to_string();
                    html.replace(r#"[##_article_rep_simple_date_##]"#, &time)
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_year_##]"#, &date_year.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_month_##]"#, &date_month.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_day_##]"#, &date_date.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_hour_##]"#, &time_hour.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_minute_##]"#, &time_hour.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_date_second_##]"#, &time_second.as_str())
                })
                .html_str_replace(|html| {
                    html.replace(r#"[##_article_rep_author_##]"#, "관리자")
                })
                .html_str_replace(|html| {
                    let mut cn = String::new();
                    if let Some(v) = category_name.deref() {
                        cn = v.clone().replace(r#"///"#, "/");
                    }

                    html.replace(r#"[##_article_rep_desc_##]"#, format!(r#"
                        <div class="tt_article_useless_p_margin contents_style">
                            {}
                        </div>

                        <div class="container_postbtn #post_button_group">
                            <div class="postbtn_like">
                                <div class="wrap_btn" id="reaction-15">
                                    <button class="btn_post uoc-icon">
                                        <div class="uoc-icon">
                                            <span class="ico_postbtn ico_like">좋아요</span>
                                            <span class="txt_like uoc-count">1</span>
                                        </div>
                                    </button>
                                </div>
                                
                                <div class="wrap_btn wrap_btn_share">
                                    <button type="button" class="btn_post sns_btn btn_share" aria-expanded="false">
                                        <span class="ico_postbtn ico_share">공유하기</span>
                                    </button>
                                    <div class="layer_post" id="tistorySnsLayer">
                                        <div class="bundle_post">
                                            <button class="btn_mark" data-service="url">
                                                <span class="ico_sns ico_url"></span>
                                                <span class="txt_sns">URL 복사</span>
                                            </button>
                                            <button class="btn_mark" data-service="kakaotalk">
                                                <span class="ico_sns ico_kt"></span>
                                                <span class="txt_sns">카카오톡 공유</span>
                                            </button>
                                            <button class="btn_mark" data-service="facebook">
                                                <span class="ico_sns ico_fb"></span>
                                                <span class="txt_sns">페이스북 공유</span>
                                            </button>
                                            <button class="btn_mark" data-service="twitter">
                                                <span class="ico_sns ico_x"></span>
                                                <span class="txt_sns">엑스 공유</span>
                                            </button>
                                            <span class="ico_postbtn ico_arrbt"></span>
                                        </div>
                                    </div>
                                </div>
                                <div class="wrap_btn">
                                    <button type="button" class="btn_post" data-entry-id="15">
                                        <span class="ico_postbtn ico_statistics">통계</span>
                                    </button>
                                </div>
                                <div class="wrap_btn wrap_btn_etc" data-entry-id="15" data-entry-visibility="public" data-category-visibility="public">
                                    <button type="button" class="btn_post btn_etc1" aria-expanded="false">
                                        <span class="ico_postbtn ico_etc">게시글 관리</span>
                                    </button>
                                    <div class="layer_post" id="tistoryEtcLayer"></div>
                                </div>
                            </div>
                            <div data-tistory-react-app="SupportButton"></div>
                        </div>

                        <div class="another_category another_category_color_gray">
                            <h4>'<a href="/category/A%20%EC%84%B8%EA%B3%84">{}</a>' 카테고리의 다른 글</h4>
                            <table>
                            <tbody><tr>
                                <th><a href="/13">테스트 5</a>&nbsp;&nbsp;<span>(0)</span></th>
                                <td>2023.10.03</td>
                            </tr>
                            <tr>
                                <th><a href="/12">테스트 4</a>&nbsp;&nbsp;<span>(0)</span></th>
                                <td>2023.10.03</td>
                            </tr>
                            <tr>
                                <th><a href="/11">테스트 3</a>&nbsp;&nbsp;<span>(0)</span></th>
                                <td>2023.10.03</td>
                            </tr>
                            <tr>
                                <th><a href="/10">테스트 2</a>&nbsp;&nbsp;<span>(0)</span></th>
                                <td>2023.10.03</td>
                            </tr>
                            <tr>
                                <th><a href="/9">테스트 1</a>&nbsp;&nbsp;<span>(0)</span></th>
                                <td>2023.10.03</td>
                            </tr>
                            </tbody></table>
                        </div>
                    "#, contents.as_str(), cn).as_str())
                })
            ;

            // s_article_next
            target
                .select(SelectOptions {
                    element_name: "s_article_next",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    let post_next_and_prev = Rc::clone(&post_next_and_prev);
                    if post_next_and_prev.0.is_none() {
                        return String::new();
                    }
                    let next_post = post_next_and_prev.0.clone().unwrap();
                    let post_id = next_post.post_id.unwrap();
                    let next_post_title = next_post.title.unwrap();
                    let mut result = matched_str_unwrap.unwrap();
                    let mut url = format!("/{}", &post_id);
                    if let PostType::Notice = next_post.post_type.unwrap() {
                        url = format!("/notice/{}", &post_id);
                    }
                    result = result.replace(r#"[##_article_next_link_##]"#, url.as_str());
                    result = result.replace(r#"[##_article_next_title_##]"#, next_post_title.as_str());
                    result
                })
                .commit()   
            ;

            // s_article_prev
            target
                .select(SelectOptions {
                    element_name: "s_article_prev",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    let post_next_and_prev = Rc::clone(&post_next_and_prev2);
                    if post_next_and_prev.1.is_none() {
                        return String::new();
                    }
                    let prev_post = post_next_and_prev.1.clone().unwrap();
                    let post_id = prev_post.post_id.unwrap();
                    let prev_post_title = prev_post.title.unwrap();
                    let mut result = matched_str_unwrap.unwrap();
                    let mut url = format!("/{}", &post_id);
                    if let PostType::Notice = prev_post.post_type.unwrap() {
                        url = format!("/notice/{}", &post_id);
                    }
                    result = result.replace(r#"[##_article_prev_link_##]"#, url.as_str());
                    result = result.replace(r#"[##_article_prev_title_##]"#, prev_post_title.as_str());
                    result
                })
                .commit()   
            ;

            // s_rp_count
            target
                .select(SelectOptions {
                    element_name: "s_rp_count",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    let mut result = matched_str_unwrap.unwrap();
                    let mut count = 0;
                    if let Some(v) = comment_list.deref() {
                        for item in v {
                            count += 1;
                            if let Some(kk) = &item.comment_list {
                                count += kk.len();
                            }
                        }
                    }
                    result = result.replace(r#"[##_article_rep_rp_cnt_##]"#, count.to_string().as_str());
                    result
                })
                .commit() 
            ;

            // s_rp
            target
                .select(SelectOptions {
                    element_name: "s_rp",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                })
                .replacer(move |_, matched_str_unwrap| {
                    let unwarp_html = matched_str_unwrap.unwrap();
                    let comment_list2 = Rc::clone(&comment_list2);
                    let mini_root = Bucket::new(&unwarp_html);
                    let is_guest2 = Rc::clone(&is_guest2);

                    // s_rp_container
                    mini_root
                        .select(SelectOptions {
                            element_name: "s_rp_container",
                            attrs: None,
                            is_attrs_check_string_contain: true,
                        })
                        .replacer(move |_, matched_str_unwrap| {
                            matched_str_unwrap.unwrap()
                        })
                        .select(SelectOptions {
                            element_name: "s_rp_rep",
                            attrs: None,
                            is_attrs_check_string_contain: true,
                        })
                        .replacer(move |_, matched_str_unwrap| {
                            let comment_list2 = Rc::clone(&comment_list2);
                            let html_template = matched_str_unwrap.unwrap();
                            let mut list_vec: Vec<String> = Vec::new();
                            if let Some(vv) = comment_list2.deref().as_ref() {
                                for item in vv {
                                    let child_comment_list = Rc::new(item.comment_list.clone());

                                    let temp = Bucket2::new(html_template.as_str());
                                    temp.delete_code_block("s_rp2_container");
                                    let mut temp_html = temp.get_html();
                                    temp_html = temp_html.replace(r#"[##_rp_rep_id_##]"#, item.comment_id.clone().unwrap().as_str());
                                    temp_html = temp_html.replace(r#"[##_rp_rep_logo_##]"#, item.profile_img_url.clone().unwrap().as_str());
                                    temp_html = temp_html.replace(r#"[##_rp_rep_name_##]"#, item.name.clone().unwrap().as_str());
                                    temp_html = temp_html.replace(r#"[##_rp_rep_date_##]"#, date_format(item.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M").as_str());
                                    temp_html = temp_html.replace(r#"[##_rp_rep_desc_##]"#, item.content.clone().unwrap().as_str());
                                    temp_html = temp_html.replace(r#"[##_rp_rep_onclick_reply_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);
                                    temp_html = temp_html.replace(r#"[##_rp_rep_onclick_delete_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);
                                    temp.set_html(temp_html);
                                    temp.delete_revoke();
                                    let revoke_html = temp.get_html();
                                    let revoke_bucket = Bucket::new(&revoke_html);
                                    revoke_bucket
                                        .select(SelectOptions {
                                            element_name: "s_rp2_container",
                                            attrs: None,
                                            is_attrs_check_string_contain: true,
                                        })
                                        .replacer(move |_, matched_str_unwrap| {
                                            matched_str_unwrap.unwrap()
                                        })
                                        .select(SelectOptions {
                                            element_name: "s_rp2_rep",
                                            attrs: None,
                                            is_attrs_check_string_contain: true,
                                        })
                                        .replacer(move |_, matched_str_unwrap| {
                                            let child_comment_list = Rc::clone(&child_comment_list);
                                            let kk = child_comment_list.deref().as_ref();
                                            if kk.is_none() {
                                                return String::new();
                                            }
                                            let h_template = matched_str_unwrap.unwrap();
                                            let child_comments = kk.unwrap();
                                            let mut li_vec: Vec<String> = Vec::new();
                                            for m_item in child_comments {
                                                let mut push_string = h_template.clone();
                                                push_string = push_string.replace(r#"[##_rp_rep_id_##]"#, m_item.comment_id.clone().unwrap().as_str());
                                                push_string = push_string.replace(r#"[##_rp_rep_logo_##]"#, m_item.profile_img_url.clone().unwrap().as_str());
                                                push_string = push_string.replace(r#"[##_rp_rep_name_##]"#, m_item.name.clone().unwrap().as_str());
                                                push_string = push_string.replace(r#"[##_rp_rep_date_##]"#, date_format(m_item.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M").as_str());
                                                push_string = push_string.replace(r#"[##_rp_rep_desc_##]"#, m_item.content.clone().unwrap().as_str());
                                                push_string = push_string.replace(r#"[##_rp_rep_onclick_reply_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);
                                                push_string = push_string.replace(r#"[##_rp_rep_onclick_delete_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);
                                                li_vec.push(push_string);
                                            }
                                            li_vec.join("")
                                        })
                                        .commit()
                                    ;
                                    list_vec.push(revoke_bucket.get_html());
                                }
                            }
                                
                            list_vec.join("")
                        })
                        .commit()
                    ;

                    // s_rp_input_form
                    mini_root
                        .select(SelectOptions {
                            element_name: "s_rp_input_form",
                            attrs: None,
                            is_attrs_check_string_contain: true,
                        })   
                        .replacer(move |_, matched_str_unwrap| {
                            let is_guest2 = Rc::clone(&is_guest2);

                            let mut result2 = matched_str_unwrap.unwrap();
                            result2 = result2.replace(r#"[##_rp_input_comment_##]"#, "comment");
                            result2 = result2.replace(r#"[##_rp_onclick_submit_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#);

                            let mini_root2 = Bucket::new(&result2);

                            // s_rp_guest
                            mini_root2
                                .select(SelectOptions {
                                    element_name: "s_rp_guest",
                                    attrs: None,
                                    is_attrs_check_string_contain: true,
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    let ig = is_guest2.deref();
                                    if let Some(ii) = ig {
                                        if ii != &true {
                                            return String::new();
                                        }
                                    }

                                    let mut result = matched_str_unwrap.unwrap();
                                    result = result.replace(r#"[##_rp_input_name_##]"#, "name");
                                    result = result.replace(r#"[##_guest_name_##]"#, "");
                                    result = result.replace(r#"[##_rp_input_password_##]"#, "password");
                                    result = result.replace(r#"[##_rp_admin_check_##]"#, "");
                                    result
                                })
                                .commit()
                            ;

                            // s_rp_member
                            mini_root2
                                .select(SelectOptions {
                                    element_name: "s_rp_member",
                                    attrs: None,
                                    is_attrs_check_string_contain: true,
                                })
                                .replacer(move |_, matched_str_unwrap| {
                                    let mut result = matched_str_unwrap.unwrap();
                                    result = result.replace(r#"[##_rp_input_is_secret_##]"#, "secret");
                                    result
                                })
                                .commit()
                            ;

                            mini_root2.get_html()
                        })
                        .commit()
                    ;

                    let namecard_html = format!(r#"
                        <div data-tistory-react-app="Namecard">
                            <div class="tt_box_namecard">
                                <div class="tt_cont">
                                    <a href="/" class="tt_tit_cont">{}</a>
                                    <a href="/" class="tt_desc">{}</a>
                                </div>
                                <a href="/" class="tt_wrap_thumb">
                                    <span class="tt_thumb_g" style="background-image: url('{}');">
                                    </span>
                                </a>
                            </div>
                        </div>
                    "#, config2.get_blog_title().unwrap(), config2.get_blog_description().unwrap(), config2.get_blog_profile_img_url().unwrap());
                    let resut_result = format!(r#"{}<div id="entry{}Comment">{}</div>"#, namecard_html, post_id, mini_root.get_html());
                    resut_result
                })
                .commit()
            ;
        }

        let post = self.config.get_post(Some(option.post_id));
        if let Some(o) = post {
            let my_post = Rc::new(o);
            match my_post.post_type.as_ref().unwrap() {
                PostType::Normal => {
                    let mini_root = Bucket::new(&s_article_permalink_rep_template);
                    common(&mini_root, &config, &my_post);
                    root
                        .html_str_replace(|html| {
                            html.replace(r#"<s_article_rep></s_article_rep>"#, mini_root.get_html().as_str())
                        })
                    ;
                },
                PostType::Notice => {
                    let mini_root = Bucket::new(&s_notice_permalink_rep_template);
                    common(&mini_root, &config, &my_post);
                    root
                        .html_str_replace(|html| {
                            html.replace(r#"<s_article_rep></s_article_rep>"#, mini_root.get_html().as_str())
                        })
                    ;
                },
                PostType::Protected => {
                    let mini_root = Bucket::new(&s_article_protected_permalink_rep_template);
                    common(&mini_root, &config, &my_post);
                    mini_root
                        .html_str_replace(|html| {
                            html.replace(r#"[##_article_dissolve_##]"#, r#"alert('본 기능은 실제 티스토리 블로그 환경에서 시도해주세요.');"#)
                        })
                    ;
                    root
                        .html_str_replace(|html| {
                            html.replace(r#"<s_article_rep></s_article_rep>"#, mini_root.get_html().as_str())
                        })
                    ;
                },
            }
        }
    }
} 

impl Replacer {
    pub fn apply_images_to_virtualcdn(&self) {
        let root = Rc::clone(&self.root);
        root
            .html_str_replace(|html| {
                html.replace(r#"./images/"#, r#"/virtualcdn/images/"#).replace(r#"'/images/"#, r#"'/virtualcdn/images/"#).replace(r#""/images/"#, r#""/virtualcdn/images/"#)
            })
            .commit();
    }

    pub fn apply_index_page(&self, option: ApplyIndexPageOptions) -> &Self {
        let post_select_option = option.apply_index_list_option.post_select_option.clone().unwrap();
        let apply_guest_book_option = ApplyGuestBookOptions {
            is_hide: true,
            guestbook_select_option: None,
        };
        let apply_tag_list_option = ApplyTagListOptions {
            is_hide: true,
        };

        self.apply_common(ApplyCommonOptions { 
            search: option.search_keyword, 
            body_id: option.body_id,
        });
        self.apply_home_cover(ApplyHomeCoverOptions {
            is_hide: !option.is_show_home_cover,
        });
        self.apply_index_list(option.apply_index_list_option);
        self.apply_guest_book(apply_guest_book_option);
        self.apply_tag_list(apply_tag_list_option);

        let mut post_select_option_clone = post_select_option.clone();
        post_select_option_clone.set_size(None);
        post_select_option_clone.set_page(None);
        self.apply_pagination(ApplyPaginationOptions {
            is_hide: option.is_show_home_cover,
            pagination_info: Some(PaginationInfo {
                base_url: option.base_url,
                total_count: self.config.get_posts(Some(post_select_option_clone)).unwrap_or_else(|| vec![]).len(),
                page: post_select_option.page.unwrap(),
                size: post_select_option.size.unwrap(),
            }),
        });
        &self
    }

    pub fn apply_tag_index_page(&self) -> &Self {
        self.apply_common(ApplyCommonOptions { 
            search: String::new(), 
            body_id: String::from("tt-body-tag"),
        });
        self.apply_home_cover(ApplyHomeCoverOptions {
            is_hide: true,
        });
        self.apply_index_list(ApplyIndexListOptions {
            is_hide: true,
            post_select_option: None,
        });
        self.apply_guest_book(ApplyGuestBookOptions { 
            is_hide: true,
            guestbook_select_option: None,
        });
        self.apply_tag_list(ApplyTagListOptions { 
            is_hide: false,
        });
        self.apply_pagination(ApplyPaginationOptions { 
            is_hide: true, 
            pagination_info: None 
        });
        &self
    }

    pub fn apply_guestbook_page(&self, option: ApplyGuestbookPageOptions) -> &Self {
        let guestbook_select_option: GuestbookSelectOption = option.guestbook_select_option;

        self.apply_common(ApplyCommonOptions { 
            search: String::new(), 
            body_id: String::from("tt-body-guestbook"),
        });
        self.apply_home_cover(ApplyHomeCoverOptions {
            is_hide: true,
        });
        self.apply_index_list(ApplyIndexListOptions {
            is_hide: true,
            post_select_option: None,
        });
        self.apply_guest_book(ApplyGuestBookOptions { 
            is_hide: false, 
            guestbook_select_option: Some(guestbook_select_option.clone()),
        });
        self.apply_tag_list(ApplyTagListOptions { 
            is_hide: true,
        });
        let mut guestbook_select_option_clone = guestbook_select_option.clone();
        guestbook_select_option_clone.set_size(None);
        guestbook_select_option_clone.set_page(None);
        self.apply_pagination(ApplyPaginationOptions {
            is_hide: false,
            pagination_info: Some(PaginationInfo {
                base_url: option.base_url,
                total_count: self.config.get_guestbooks(Some(guestbook_select_option_clone)).len(),
                page: guestbook_select_option.page.unwrap(),
                size: guestbook_select_option.size.unwrap(),
            }),
        });
        &self
    }

    pub fn apply_post_permalink_page(&self, option: ApplyPostPermalinkPageOptions) -> &Self {
        let apply_post_permalink = option.apply_post_permalink;
        self.apply_common(ApplyCommonOptions { 
            search: String::new(), 
            body_id: String::from("tt-body-page"),
        });
        self.apply_home_cover(ApplyHomeCoverOptions {
            is_hide: true
        });
        if let Some(v) = apply_post_permalink {
            self.apply_post_permalink(v);
        }
        self.apply_index_list(ApplyIndexListOptions {
            is_hide: true,
            post_select_option: None,
        });
        self.apply_guest_book(ApplyGuestBookOptions { 
            is_hide: true,
            guestbook_select_option: None,
        });
        self.apply_tag_list(ApplyTagListOptions { 
            is_hide: true,
        });
        self.apply_pagination(ApplyPaginationOptions { 
            is_hide: true, 
            pagination_info: None 
        });
        &self
    }
}

struct ApplyCommonOptions {
    search: String,
    body_id: String,
}

pub struct ApplyHomeCoverOptions {
    is_hide: bool,
}

pub struct ApplyIndexPageOptions {
    pub search_keyword: String,
    pub base_url: String,
    pub body_id: String,
    pub apply_index_list_option: ApplyIndexListOptions,
    pub is_show_home_cover: bool,
    // pub apply_guest_book_option: ApplyGuestBookOptions,
    // pub apply_tag_list_option: ApplyTagListOptions,
    // pub apply_pagination: ApplyPaginationOptions,
}

pub struct ApplyGuestbookPageOptions {
    pub base_url: String,
    pub guestbook_select_option: GuestbookSelectOption,
}

pub struct ApplyPostPermalinkPageOptions {
    pub apply_post_permalink: Option<ApplyPostPermalink>,
}

pub struct ApplyPostPermalink {
    pub post_id: String,
}

pub struct ApplyIndexListOptions {
    pub is_hide: bool,
    pub post_select_option: Option<PostSelectOption>,
}

pub struct ApplyGuestBookOptions {
    pub is_hide: bool,
    pub guestbook_select_option: Option<GuestbookSelectOption>,
}

pub struct ApplyTagListOptions {
    pub is_hide: bool,
}

pub struct ApplyPaginationOptions {
    pub is_hide: bool,
    pub pagination_info: Option<PaginationInfo>,
}

pub struct PaginationInfo {
    pub base_url: String,
    pub total_count: usize,
    pub page: u32,
    pub size: u32,
}
