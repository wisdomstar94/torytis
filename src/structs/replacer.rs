use std::{ops::Deref, rc::Rc};
use chrono::NaiveDateTime;
use html_regex::{html_string_root_element_unwrap, select_from_html_string_one, Bucket, SelectOptions};

use crate::{common::get_pagination_calculate, structs::torytis_dev_config::{Post, PostType, TorytisDevConfig}};

use super::torytis_dev_config::{get_skin_variable_info_map, PostSelectOption};

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
        let recent_notice_list = Rc::new(config.get_recent_notice_list().clone().unwrap());

        let options_search = options.search;
        let body_id = options.body_id;

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
                html.replace(r#"<link href="./style.css" type="text/css" rel="stylesheet" />"#, r#"<link href="/virtualcdn/style.css" type="text/css" rel="stylesheet" />"#)
            })
            .html_str_replace(|html| {
                html.replace(r#"<script src="./images/script.js"></script>"#, r#"<script src="/virtualcdn/images/script.js"></script>"#)
            })
            .select(SelectOptions {
                element_name: "s_sidebar",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(|_, unwrap_matched_str| {
                unwrap_matched_str.unwrap()
            })
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
                            bucket
                                .html_str_replace(|s| {
                                    s.replace(r#"[##_rctrp_rep_name_##]"#, &name)
                                })
                                .html_str_replace(|s| {
                                    let time = NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").unwrap().format("%m.%d").to_string();
                                    s.replace(r#"[##_rctrp_rep_time_##]"#, &time)
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

    fn apply_home_cover(&self) {
        let root = Rc::clone(&self.root);
        // let config = self.config.get_clone_rc();

        root
            .select(SelectOptions {
                element_name: "s_cover_group",
                attrs: None,
                is_attrs_check_string_contain: true
            })
            .replacer(|_, _| {
                String::new()  
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
        
        fn common(root: &Bucket, item: Post) {
            let post_type = &item.post_type;
            let thumbnail_img_url1 = item.thumbnail_img_url.as_ref().unwrap().clone();
            let thumbnail_img_url2 = item.thumbnail_img_url.as_ref().unwrap().clone();
            let category_name = item.category_name.as_ref().unwrap().clone();
            let title = item.title.as_ref().unwrap().clone();
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
                    let mut result = matched_str_unwrap.unwrap();
                    result = result.replace(r#"[##_article_rep_thumbnail_url_##]"#, &thumbnail_img_url1.clone());
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
                                common(&root, item.clone());
                                list_vec.push(root.get_html());
                            },
                            // 공지사항 글
                            super::torytis_dev_config::PostType::Notice => {
                                let root = Bucket::new(&notice_index_rep_template);
                                common(&root, item.clone());
                                list_vec.push(root.get_html());
                            },
                            // 암호로 보호된 글
                            super::torytis_dev_config::PostType::Protected => {
                                let root = Bucket::new(&protected_index_rep_template);
                                common(&root, item.clone());
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

        root    
            .select(SelectOptions {
                element_name: "s_guest",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, _| {
                if is_hide {
                    return String::new();
                }

                todo!()
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
} 

impl Replacer {
    pub fn apply_index_page(&self, option: ApplyIndexPageOptions) -> &Self {
        let post_select_option = option.apply_index_list_option.post_select_option.clone().unwrap();
        let apply_guest_book_option = ApplyGuestBookOptions {
            is_hide: true,
        };
        let apply_tag_list_option = ApplyTagListOptions {
            is_hide: true,
        };

        self.apply_common(ApplyCommonOptions { 
            search: option.search_keyword, 
            body_id: option.body_id,
        });
        self.apply_home_cover();
        self.apply_index_list(option.apply_index_list_option);
        self.apply_guest_book(apply_guest_book_option);
        self.apply_tag_list(apply_tag_list_option);

        let mut post_select_option_clone = post_select_option.clone();
        post_select_option_clone.set_size(None);
        post_select_option_clone.set_page(None);
        self.apply_pagination(ApplyPaginationOptions {
            is_hide: false,
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
        self.apply_home_cover();
        self.apply_index_list(ApplyIndexListOptions {
            is_hide: true,
            post_select_option: None,
        });
        self.apply_guest_book(ApplyGuestBookOptions { 
            is_hide: true 
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
}

struct ApplyCommonOptions {
    search: String,
    body_id: String,
}

pub struct ApplyIndexPageOptions {
    pub search_keyword: String,
    pub base_url: String,
    pub body_id: String,
    pub apply_index_list_option: ApplyIndexListOptions,
    // pub apply_guest_book_option: ApplyGuestBookOptions,
    // pub apply_tag_list_option: ApplyTagListOptions,
    // pub apply_pagination: ApplyPaginationOptions,
}

pub struct ApplyIndexListOptions {
    pub is_hide: bool,
    pub post_select_option: Option<PostSelectOption>,
}

pub struct ApplyGuestBookOptions {
    pub is_hide: bool,
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