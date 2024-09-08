use std::{collections::HashMap, ops::Deref, rc::Rc};

use chrono::NaiveDateTime;
use html_regex::{Bucket, SelectOptions};
use serde::Deserialize;
use xmltree::Element;

use crate::common::{get_index_xml_content, get_torytis_dev_config_json_content};

#[derive(Deserialize, Debug, Clone)]
pub struct TorytisDevConfig {
    is_guest: Option<bool>,
    blog_title: Option<String>,
    blog_description: Option<String>,
    blog_profile_name: Option<String>,
    blog_profile_img_url: Option<String>,
    visitor: Option<Visitor>,
    posts: Option<Vec<Post>>,
    // recent_comment_list: Option<Vec<RecentComment>>,
    skin_home_cover: Option<SkinHomeCover>,
    category_list: Option<Vec<Category>>,
    skin_setting_variables: Option<HashMap<String, String>>,
    guestbook_list: Option<Vec<GuestBook>>,
}

impl TorytisDevConfig {
    pub fn new() -> Self {
        let json_string = get_torytis_dev_config_json_content();
        let result = serde_json::from_str::<TorytisDevConfig>(&json_string).unwrap();
        result.valid_check();
        result
    }

    pub fn get_clone_rc(&self) -> Rc<Self> {
        Rc::new(self.clone())
    }

    pub fn get_is_guest(&self) -> Option<bool> {
        self.is_guest.clone()
    }

    pub fn get_guestbook_list(&self) -> Vec<GuestBook> {
        let mut result: Vec<GuestBook> = Vec::new();
        if let Some(guestbook_list) = &self.guestbook_list {
            for item in guestbook_list {
                result.push(item.clone());
            }
        }
        result
    }

    pub fn get_blog_title(&self) -> Option<&str> {
        if let Some(v) = &self.blog_title {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_blog_description(&self) -> Option<&str> {
        if let Some(v) = &self.blog_description {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_blog_profile_name(&self) -> Option<&str> {
        if let Some(v) = &self.blog_profile_name {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_blog_profile_img_url(&self) -> Option<&str> {
        if let Some(v) = &self.blog_profile_img_url {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_visitor(&self) -> Option<&Visitor> {
        if let Some(v) = &self.visitor {
            Some(v)
        } else {
            None
        }
    }

    // pub fn get_recent_comment_list(&self) -> Option<&Vec<RecentComment>> {
    //     if let Some(v) = &self.recent_comment_list {
    //         Some(v)
    //     } else {
    //         None
    //     }
    // }

    pub fn get_category_list(&self) -> Option<&Vec<Category>> {
        if let Some(v) = &self.category_list {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_category_list_html(&self) -> String {
        let category_list = self.get_category_list().unwrap().clone();
        let html = r#"
            <ul class="tt_category">
                <li class="">
                    <a href="/category" class="link_tit">
                        분류 전체보기 <span class="c_cnt">(212)</span>
                    </a>
                    <ul class="category_list">
                        <s_category_item>
                            <li class="">
                                <a href="/category/[##_category_name_##]" class="link_item">
                                    [##_category_name_##] <span class="c_cnt">(25)</span>
                                </a>
                                <s_sub_category_rep>
                                    <ul class="sub_category_list">
                                        <s_sub_category_item>
                                            <li class="">
                                                <a href="/category/[##_category_name2_##]/[##_sub_category_name2_##]" class="link_sub_item">
                                                    [##_sub_category_name2_##] <span class="c_cnt">(10)</span>
                                                </a>
                                            </li>
                                        </s_sub_category_item>
                                    </ul>
                                </s_sub_category_rep>
                            </li>
                        </s_category_item>
                    </ul>
                </li>
            </ul>
        "#;
        let root = Bucket::new(html);
        root  
            .select(SelectOptions {
                element_name: "s_category_item",
                attrs: None,
                is_attrs_check_string_contain: true,
            })
            .replacer(move |_, matched_str_unwrap| {
                let template_html =  matched_str_unwrap.unwrap();
                let mut list_html_vec: Vec<String> = Vec::new();
                for item in &category_list {
                    let sub_category_list = Rc::new(item.category_list.clone());
                    let sub_category_list1 = Rc::clone(&sub_category_list);
                    let sub_category_list2 = Rc::clone(&sub_category_list);
                    let parent_category_name = Rc::new(item.name.clone());
                    
                    let mini_root = Bucket::new(&template_html);
                    mini_root
                        .html_str_replace(|h| {
                            h.replace(r#"[##_category_name_##]"#, item.name.as_str())
                        })
                        .select(SelectOptions { 
                            element_name: "s_sub_category_rep", 
                            attrs: None, 
                            is_attrs_check_string_contain: true 
                        })
                        .replacer(move |_, matched_str_unwrap| {
                            if let Some(_) = Rc::clone(&sub_category_list1).deref() {
                                matched_str_unwrap.unwrap()
                            } else {
                                String::new()
                            }
                        })
                        .select(SelectOptions { 
                            element_name: "s_sub_category_item", 
                            attrs: None, 
                            is_attrs_check_string_contain: true 
                        })
                        .replacer(move |_, matched_str_unwrap| {
                            let template_html =  matched_str_unwrap.unwrap();
                            let mut list_html_vec2: Vec<String> = Vec::new();
                            if let Some(v) = Rc::clone(&sub_category_list2).deref() {
                                for sub_category in v {
                                    let sub_category_name = Rc::new(sub_category.name.clone());
                                    let m_root = Bucket::new(&template_html);
                                    m_root
                                        .html_str_replace(|s| {
                                            let pn = parent_category_name.clone();
                                            s.replace(r#"[##_category_name2_##]"#, pn.as_str())
                                        })
                                        .html_str_replace(|s| {
                                            s.replace(r#"[##_sub_category_name2_##]"#, sub_category_name.as_str())
                                        })
                                    ;
                                    list_html_vec2.push(m_root.get_html());
                                }
                            } 
                            list_html_vec2.join("")
                        })
                        .commit()
                    ;
                    list_html_vec.push(mini_root.get_html());
                }
                list_html_vec.join("")
            })
            .commit()
        ;
        root.get_html()
    }

    pub fn get_recent_comment_list(&self) -> Option<Vec<Comment>> {
        if let Some(v) = &self.posts {   
            let mut all_comment_list: Vec<Comment> = Vec::new();
            for post in v {
                if let Some(s) = &post.comment_list {
                    for parent_comment in s {
                        let mut parent_comment_clone = parent_comment.clone();
                        parent_comment_clone.comment_list = None;
                        all_comment_list.push(parent_comment_clone);
                        if let Some(p) = &parent_comment.comment_list {
                            for child_comment in p {
                                let child_comment_clone = child_comment.clone();
                                all_comment_list.push(child_comment_clone);
                            }
                        }
                    }
                }
            }
            all_comment_list.sort_by(|a, b| {
                // Utc (&date_str, "%Y-%m-%d %H:%M:%S");
                let a1 = NaiveDateTime::parse_from_str(a.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap().and_utc().timestamp_millis();
                let b1 = NaiveDateTime::parse_from_str(b.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap().and_utc().timestamp_millis();
                // a1.cmp(&b1)
                b1.cmp(&a1)
            });
            // println!("all_comment_list : {:#?}", all_comment_list);
            let m = all_comment_list.iter().take(5).map(|f| -> Comment {
                f.clone()
            }).collect::<Vec<Comment>>();
            Some(m)
        } else {
            None
        }
    }

    pub fn get_recent_notice_list(&self) -> Option<Vec<Post>> {
        let mut result:Option<Vec<Post>> = None;
        let list = self.get_posts(Some(PostSelectOption { 
            page: None, 
            size: None, 
            post_type: Some(PostType::Notice), 
            category_name: None, 
            sub_category_name: None,
            tag_name: None,
            title: None,
            post_id: None,
        }));
        if let Some(v) = list {
            result = Some(v.iter().take(5).map(|s| s.clone()).collect::<Vec<Post>>())
        }
        result
    }

    pub fn get_post_id_from_comment_id(posts: Option<Vec<Post>>, comment_id: &str) -> Option<String> {
        let mut result: Option<String> = None;
        let posts = posts.clone().unwrap_or_else(|| vec![]);
        for post in posts {
            let post_id = post.post_id.unwrap_or_else(|| String::new());
            for parent_comment in post.comment_list.unwrap_or_else(|| vec![]) {
                if parent_comment.comment_id.unwrap_or_else(|| String::new()).as_str() == comment_id {
                    result = Some(post_id.clone());
                    break;
                }
                for child_comment in parent_comment.comment_list.unwrap_or_else(|| vec![]) {
                    if child_comment.comment_id.unwrap_or_else(|| String::new()).as_str() == comment_id {
                        result = Some(post_id.clone());
                        break;
                    }
                }
                if let Some(_) = result {
                    break;
                }
            }
        }
        result
    }

    pub fn get_posts(&self, select_option: Option<PostSelectOption>) -> Option<Vec<Post>> {
        // println!("**get_posts.select_option {:#?}", select_option);
        let mut posts: Option<Vec<Post>> = None;
        if let Some(v) = &self.posts {
            let v = v.clone();
            let filterd_iter = v.iter().filter(|x| -> bool {
                let mut required_option_count = 0;
                let mut required_option_matched_count = 0;
                let mut is_allow = true;

                if let Some(select_option) = &select_option {
                    if let Some(post_type) = &select_option.post_type {
                        required_option_count += 1;
                        if let Some(this_post_type) = &x.post_type {
                            if post_type.is_equal(&this_post_type) {
                                required_option_matched_count += 1;
                            }
                        }
                    }
                    if select_option.category_name.is_some() && select_option.sub_category_name.is_some() {
                        required_option_count += 1;
                        let category_name = select_option.category_name.as_ref().unwrap();
                        let sub_category_name = select_option.sub_category_name.as_ref().unwrap();
                        if let Some(this_category_name) = &x.category_name {
                            let category_info: Vec<&str> = this_category_name.split("///").collect();
                            if let (Some(cn), Some(scn)) = (category_info.get(0), category_info.get(1)) {
                                if *cn == category_name.as_str() && *scn == sub_category_name.as_str() {
                                    required_option_matched_count += 1;
                                }
                            }
                        }
                    } else if select_option.category_name.is_some() && select_option.sub_category_name.is_none() {
                        required_option_count += 1;
                        let category_name = select_option.category_name.as_ref().unwrap();
                        if let Some(this_category_name) = &x.category_name {
                            if this_category_name.as_str() == category_name.as_str() || this_category_name.as_str().contains(format!(r#"{}///"#, category_name.as_str()).as_str()) {
                                required_option_matched_count += 1;
                            }
                        }
                    }

                    if let Some(tag_name) = &select_option.tag_name {
                        required_option_count += 1;
                        if let Some(this_tag_list) = &x.tag_list {
                            if this_tag_list.contains(tag_name) {
                                required_option_matched_count += 1;
                            }
                        }
                    }

                    if let Some(title) = &select_option.title {
                        required_option_count += 1;
                        if let Some(this_title) = &x.title {
                            if this_title.contains(title) {
                                required_option_matched_count += 1;
                            }
                        }
                    }

                    if let Some(post_id) = &select_option.post_id {
                        required_option_count += 1;
                        if let Some(this_post_id) = &x.post_id {
                            if this_post_id == post_id {
                                required_option_matched_count += 1;
                            }
                        }
                    }
                }

                if required_option_count > 0 {
                    is_allow = required_option_count == required_option_matched_count;
                }

                // if let (Some(pt), Some(pp)) = (post_type, &x.post_type) {
                //     is_allow = pt.is_equal(pp);
                // }
                is_allow
            }).map(|z| z.clone());
            let mut filterd_vec: Vec<Post> = filterd_iter.collect::<Vec<Post>>();
            
            if let Some(select_option) = select_option {
                if let (Some(page), Some(size)) = (select_option.page, select_option.size) {
                    let start_index = (page - 1) * size;
                    let end_index = start_index + size - 1;
                    let filterd_vec_clone = filterd_vec.clone();
                    // println!("start_index: {}, end_index: {}", start_index, end_index);
                    filterd_vec = Vec::new();
                    let mut index = 0;
                    for item in filterd_vec_clone {
                        if index >= start_index && index <= end_index {
                            filterd_vec.push(item);      
                        }
                        index += 1;
                    }
                    // println!("filterd_vec: {:#?}", filterd_vec);
                }
            }

            posts = Some(filterd_vec);
        }
        posts
    }

    pub fn get_post(&self, post_id: Option<String>) -> Option<Post> {
        let mut result: Option<Post> = None;
        let posts = self.get_posts(Some(PostSelectOption {
            page: None,
            size: None,
            post_type: None,
            category_name: None,
            sub_category_name: None,
            tag_name: None,
            title: None,
            post_id,
        }));
        if let Some(v) = posts {
            if let Some(k) = v.get(0) {
                result = Some(k.clone());
            }
        }
        result
    }

    pub fn get_next_and_prev_post(&self, post_id: Option<String>) -> (Option<Post>, Option<Post>) {
        if post_id.is_none() {
            return (None, None);
        }
        let post_id = post_id.unwrap();

        let mut next_post: Option<Post> = None;
        let mut prev_post: Option<Post> = None;

        let posts_wrap = self.get_posts(None);
        if let Some(posts) = &posts_wrap {
            let mut index = 0;
            let mut target_post_index = 0;
            let mut target_post_category_name = String::new();

            for post in posts {
                if post.post_id.clone().unwrap() == post_id {
                    target_post_index = index;
                    target_post_category_name = post.category_name.clone().unwrap();
                    break;
                }
                index += 1;
            }

            let mut next_post_index = 99999;
            let mut current_index: u32 = 0;
            let max_index: i32 = target_post_index - 1;
            loop {
                if next_post_index != 99999 {
                    break;
                }

                if current_index as i32 > max_index {
                    break;
                }

                let post = posts.get(current_index as usize);
                if let Some(p) = post {
                    if p.category_name.clone().unwrap() == target_post_category_name {
                        next_post_index = current_index;
                        break;
                    }
                }

                current_index += 1;
            }

            let mut prev_post_index = 99999;
            let mut current_index: u32 = target_post_index as u32 + 1;
            let max_index: i32 = posts.len() as i32 - 1;
            loop {
                if prev_post_index != 99999 {
                    break;
                }

                if current_index as i32 > max_index {
                    break;
                }

                let post = posts.get(current_index as usize);
                if let Some(p) = post {
                    if p.category_name.clone().unwrap() == target_post_category_name {
                        prev_post_index = current_index;
                        break;
                    }
                }

                current_index += 1;
            }

            next_post = posts.get(next_post_index as usize).cloned();
            prev_post = posts.get(prev_post_index as usize).cloned();
        }

        (next_post, prev_post)
    }

    pub fn get_guestbooks(&self, select_option: Option<GuestbookSelectOption>) -> Vec<GuestBook> {
        let mut result: Vec<GuestBook> = Vec::new();
        if let Some(v) = &self.guestbook_list {
            let filterd_iter = v.iter().map(|s| s.clone());
            let mut filterd_vec = filterd_iter.collect::<Vec<GuestBook>>();
            if let Some(select_option) = select_option {
                if let (Some(page), Some(size)) = (select_option.page, select_option.size) {
                    let start_index = (page - 1) * size;
                    let end_index = start_index + size - 1;
                    let filterd_vec_clone = filterd_vec.clone();
                    // println!("start_index: {}, end_index: {}", start_index, end_index);
                    filterd_vec = Vec::new();
                    let mut index = 0;
                    for item in filterd_vec_clone {
                        if index >= start_index && index <= end_index {
                            filterd_vec.push(item);      
                        }
                        index += 1;
                    }
                    // println!("filterd_vec: {:#?}", filterd_vec);
                }
            }
            result = filterd_vec;
        }
        result
    }

    pub fn get_tag_unique_list(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let posts = self.get_posts(None);
        if let Some(posts) = posts {
            for item in posts {
                if let Some(tag_list) = item.tag_list {
                    for tag in tag_list {
                        if !result.contains(&tag) {
                            result.push(tag);
                        }
                    }
                }
            }
        }
        result  
    }

    pub fn get_skin_home_cover(&self) -> Option<SkinHomeCover> {
        self.skin_home_cover.clone()
    }

    pub fn get_xml_cover_items(&self) -> Vec<XmlCoverItem> {
        let mut vec: Vec<XmlCoverItem> = vec![];
        let src_public_index_xml_content = get_index_xml_content();
        let element = Element::parse(src_public_index_xml_content.as_bytes()).unwrap();
        for item_node in &element.get_child("cover").unwrap().children {
            let item_element = item_node.as_element().unwrap();
            let name = item_element.get_child("name").unwrap().get_text().unwrap().to_string();
            let label = item_element.get_child("label").unwrap().get_text().unwrap().to_string();
            let description = item_element.get_child("description").unwrap().get_text().unwrap().to_string();
            vec.push(XmlCoverItem {
                name,
                label,
                description
            });
        }
        vec
    }

    pub fn get_posts_from_category_name(&self, category_name: &str) -> Vec<Post> {
        // let mut result: Vec<Post> = vec![];
        // let categorys = category_name.split("///").collect::<Vec<&str>>();
        // let is_exist_subcategory = categorys.len() == 2;
        let posts = self.get_posts(None).unwrap_or_else(|| vec![]);
        let result: Vec<Post> = posts.iter().filter(|p| {
            p.category_name.clone().unwrap() == category_name
        }).map(|s| s.clone()).collect::<Vec<Post>>();
        result
    }

    pub fn get_skin_setting_variables(&self) -> Option<HashMap<String, String>> {
        self.skin_setting_variables.clone()
    }

    pub fn valid_check(&self) {
        // category 중복 확인
        if let Some(v) = self.category_list.clone() {
            let mut parent_category_name_list: Vec<String> = v.iter().map(|x| -> String {
                String::from(&x.name)
            }).collect();
            parent_category_name_list.sort_unstable();
            parent_category_name_list.dedup();

            if v.len() != parent_category_name_list.len() {
                panic!("torytis-dev.config.json 파일에 기재된 category_list 중에 중복된 카테고리명이 존재합니다! 중복되지 않게 수정해주세요!");
            }

            for parent_category in &v {
                if let Some(k) = &parent_category.category_list {
                    let mut child_category_list: Vec<String> = k.iter().map(|x| -> String {
                        String::from(&x.name)
                    }).collect();
                    child_category_list.sort_unstable();
                    child_category_list.dedup();

                    if k.len() != child_category_list.len() {
                        panic!("torytis-dev.config.json 파일에 기재된 category_list 중에 중복된 카테고리명이 존재합니다! 중복되지 않게 수정해주세요!");
                    }
                }
            }
            // for child_category_list 
            // let parent_category_name_list.collect::<&str>();
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
    pub category_list: Option<Vec<Category>>,
    pub is_new: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Visitor {
    pub count_total: Option<u64>,
    pub count_today: Option<u64>,
    pub count_yesterday: Option<u64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Content {
    r#type: Option<PostContentType>,
    value: Option<String>,
    values: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Comment {
    pub comment_id: Option<String>,
    pub name: Option<String>,
    pub profile_img_url: Option<String>,
    pub content: Option<String>,
    pub datetime: Option<String>,
    pub comment_list: Option<Vec<Comment>>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PostType {
    Normal,
    Notice,
    Protected,
}

impl PostType {
    pub fn is_equal(&self, p: &PostType) -> bool {
        match self {
            PostType::Normal => {
                if let PostType::Normal = p {
                    return true;
                }
            },
            PostType::Notice => {
                if let PostType::Notice = p {
                    return true;
                }
            },
            PostType::Protected => {
                if let PostType::Protected = p {
                    return true;
                }
            },
        }
        false
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub category_name: Option<String>,
    pub tag_list: Option<Vec<String>>,
    pub post_id: Option<String>, 
    pub post_type: Option<PostType>,
    pub is_private: Option<bool>,
    pub is_require_password: Option<bool>,
    pub require_password: Option<String>,
    pub created_at: Option<String>,
    pub title: Option<String>,
    pub thumbnail_img_url: Option<String>,
    pub contents: Option<Vec<Content>>,
    pub comment_list: Option<Vec<Comment>>,
}

impl Post {
    pub fn get_contents_summary(&self) -> String {
        let mut strings: Vec<String> = vec![];
        for item in self.contents.clone().unwrap_or_else(|| vec![]) {
            if let Some(content_type) = item.r#type {
                match content_type {
                    PostContentType::Paragraph => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::Image => {
                        // strings.push(value)
                    },
                    PostContentType::H1 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::H2 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::H3 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::H4 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::H5 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::H6 => {
                        strings.push(item.value.unwrap_or_else(|| String::new()));
                    },
                    PostContentType::Codeblock => {
                        for k in item.values.unwrap() {
                            strings.push(k);
                        }
                    },
                }
            }
            
        }
        strings.join("")
    }

    pub fn get_contents(&self) -> String {
        let mut list_vec: Vec<String> = Vec::new();
        for item in &self.contents.clone().unwrap() {
            match item.r#type.clone().unwrap() {
                PostContentType::Paragraph => {
                    let html = format!(r#"<p data-ke-size="size16" data-original-color="">{}</p>"#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::Image => {
                    let html = format!(r#"
                        <figure class="imageblock alignCenter" data-ke-mobilestyle="widthOrigin" data-filename="test-image.jpeg" data-origin-width="225" data-origin-height="225">
                            <span data-url="https://blog.kakaocdn.net/dn/NGOAu/btsAxZe9TJE/HRLbq1QU8UlHP8FhOQlKE0/img.jpg" data-lightbox="lightbox" data-original-color="">
                                <img src="{}" onerror="this.onerror=null;" data-filename="test-image.jpeg" data-origin-width="225" data-origin-height="225">
                            </span>
                        </figure>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H1 => {
                    let html = format!(r#"
                        <h1>{}</h1>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H2 => {
                    let html = format!(r#"
                        <h2 data-ke-size="size26">{}</h2>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H3 => {
                    let html = format!(r#"
                        <h3 data-ke-size="size23">{}</h3>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H4 => {
                    let html = format!(r#"
                        <h4 data-ke-size="size20">{}</h4>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H5 => {
                    let html = format!(r#"
                        <h5>{}</h5>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::H6 => {
                    let html = format!(r#"
                        <h6>{}</h6>
                    "#, item.value.clone().unwrap());
                    list_vec.push(html);
                },
                PostContentType::Codeblock => {
                    // let html = r#"
                    //     <pre id="code_1706856177241" class="javascript" data-ke-language="javascript" data-ke-type="codeblock">
                    //         <code class="hljs language-javascript">
                    //             <table class="hljs-ln">
                    //                 <tbody>
                    //                     <tr>
                    //                         <td class="hljs-ln-line hljs-ln-numbers" data-line-number="1">
                    //                             <div class="hljs-ln-n" data-line-number="1"></div>
                    //                         </td>
                    //                         <td class="hljs-ln-line hljs-ln-code" data-line-number="1">
                    //                             <span class="hljs-keyword" data-original-color="">function</span> <span class="hljs-title function_" data-original-color="">main</span>(<span class="hljs-params" data-original-color=""></span>) {
                    //                         </td>
                    //                     </tr>
                    //                     <tr>
                    //                         <td class="hljs-ln-line hljs-ln-numbers" data-line-number="2">
                    //                             <div class="hljs-ln-n" data-line-number="2"></div>
                    //                         </td>
                    //                         <td class="hljs-ln-line hljs-ln-code" data-line-number="2">    
                    //                             <span class="hljs-variable language_" data-original-color="">console</span>.<span class="hljs-title function_" data-original-color="">log</span>(<span class="hljs-string" data-original-color="">"안녕하세요!"</span>);
                    //                         </td>
                    //                     </tr>
                    //                     <tr>
                    //                         <td class="hljs-ln-line hljs-ln-numbers" data-line-number="3">
                    //                             <div class="hljs-ln-n" data-line-number="3"></div>
                    //                         </td>
                    //                         <td class="hljs-ln-line hljs-ln-code" data-line-number="3">
                    //                             }
                    //                         </td>
                    //                     </tr>
                    //                 </tbody>
                    //             </table>
                    //         </code>
                    //     </pre>
                    // "#;
                    // list_vec.push(html.to_string());  
                    let mut html = String::from(r#"
                        <pre id="code_1706856177241" class="css" data-ke-language="css" data-ke-type="codeblock">
                            <code class="">[##_toryris_replace_place_##]</code>
                        </pre>
                    "#);
                    let mut l_v: Vec<String> = Vec::new();
                    for k in item.values.clone().unwrap() {
                        let temp_template = format!("{}\n", k);
                        l_v.push(temp_template);
                    }
                    let trs = l_v.join("");
                    html = html.replace(r#"[##_toryris_replace_place_##]"#, trs.as_str());
                    list_vec.push(html);  
                },
            }
        }
        list_vec.join("")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum PostContentType {
    Paragraph,
    Image,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Codeblock,
}

// #[derive(Deserialize, Debug, Clone)]
// pub struct RecentComment {
//     pub name: Option<String>,
//     pub date: Option<String>,
//     pub content: Option<String>,
// }

#[derive(Deserialize, Debug, Clone)]
pub struct SkinHomeCover {
    pub is_active: Option<bool>,
    pub cover_items: Option<Vec<CoverItem>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoverItem {
    pub cover_name: Option<String>,
    pub cover_title: Option<String>,
    pub cover_category_name: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct XmlCoverItem {
    pub name: String,
    pub label: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PostSelectOption {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub post_type: Option<PostType>,
    pub category_name: Option<String>,
    pub sub_category_name: Option<String>,
    pub tag_name: Option<String>,
    pub title: Option<String>,
    pub post_id: Option<String>,
}

impl PostSelectOption {
    pub fn set_page(&mut self, p: Option<u32>) {
        self.page = p;
    }

    pub fn set_size(&mut self, s: Option<u32>) {
        self.size = s;
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GuestbookSelectOption {
    pub page: Option<u32>,
    pub size: Option<u32>,
}

impl GuestbookSelectOption {
    pub fn set_page(&mut self, p: Option<u32>) {
        self.page = p;
    }

    pub fn set_size(&mut self, s: Option<u32>) {
        self.size = s;
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SkinVariableInfo {
    pub var_label: String,
    pub var_description: String,
    pub var_name: String,
    pub var_code_name: String,
    pub var_type: String,
    pub default: Option<String>,
}   

pub fn get_skin_variable_info_map() -> HashMap<String, SkinVariableInfo> {
    let mut result: HashMap<String, SkinVariableInfo> = HashMap::new();
    let index_xml_content = get_index_xml_content();

    let element = Element::parse(index_xml_content.as_bytes()).unwrap();
    let variables = element.get_child("variables").unwrap();
    let variables_list = &variables.children;
    for variable_group in variables_list {
        if let Some(variable_group_element) = variable_group.as_element() {
            let variable = &variable_group_element.children;
            for item in variable {
                if let Some(variable_element) = item.as_element() {
                    let name = variable_element.get_child("name").unwrap().get_text().unwrap().to_string();
                    let label = variable_element.get_child("label").unwrap().get_text().unwrap().to_string();
                    let description = variable_element.get_child("description").unwrap().get_text().unwrap().to_string();
                    let r#type = variable_element.get_child("type").unwrap().get_text().unwrap().to_string();
                    let mut default: Option<String> = None;
                    if let Some(variable_element) = variable_element.get_child("default") {
                        if let Some(text) = variable_element.get_text() {
                            default = Some(text.to_string());
                        }
                    }
                    let var_code_name = format!("[##_var_{}_##]", name);
                    result.insert(
                        var_code_name.to_owned(), 
                        SkinVariableInfo { 
                            var_label: label,
                            var_description: description,
                            var_name: name,
                            var_code_name,
                            var_type: r#type,
                            default,
                        }
                    );
                }
            }
        }
    }
    result
}

#[derive(Deserialize, Debug, Clone)]
pub struct GuestBook {
    pub name: Option<String>,
    pub guest_rep_id: Option<String>,
    pub guest_rep_logo: Option<String>,
    pub created_at: Option<String>,
    pub content: Option<String>,
    pub guestbook_list: Option<Vec<GuestBook>>,
}