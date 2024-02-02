use std::{ops::Deref, rc::Rc};

use chrono::NaiveDateTime;
use html_regex::{Bucket, SelectOptions};
use serde::Deserialize;
use xmltree::Element;

use crate::common::{get_index_xml_content, get_torytis_dev_config_json_content};

#[derive(Deserialize, Debug, Clone)]
pub struct TorytisDevConfig {
    blog_title: Option<String>,
    blog_description: Option<String>,
    visitor: Option<Visitor>,
    posts: Option<Vec<Post>>,
    // recent_comment_list: Option<Vec<RecentComment>>,
    skin_home_cover: Option<SkinHomeCover>,
    category_list: Option<Vec<Category>>,
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
                let a1 = NaiveDateTime::parse_from_str(a.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap().timestamp_millis();
                let b1 = NaiveDateTime::parse_from_str(b.datetime.clone().unwrap().as_str(), "%Y-%m-%d %H:%M:%S").unwrap().timestamp_millis();
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
        let list = self.get_posts(Some(&PostType::Notice));
        if let Some(v) = list {
            result = Some(v.iter().take(5).map(|s| s.clone()).collect::<Vec<Post>>())
        }
        result
    }

    pub fn get_post_id_from_comment_id(&self, comment_id: &str) -> Option<String> {
        let mut result: Option<String> = None;
        let posts = self.posts.clone().unwrap_or_else(|| vec![]);
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

    pub fn get_posts(&self, post_type: Option<&PostType>) -> Option<Vec<Post>> {
        let mut posts: Option<Vec<Post>> = None;
        if let Some(v) = &self.posts {
            let v = v.clone();
            let filterd_iter = v.iter().filter(|x| -> bool {
                let mut is_allow = true;
                if let (Some(pt), Some(pp)) = (post_type, &x.post_type) {
                    is_allow = pt.is_equal(pp);
                }
                is_allow
            }).map(|z| z.clone());
            posts = Some(filterd_iter.collect::<Vec<Post>>());
        }
        posts
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
        }
        false
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub category_name: Option<String>,
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
                }
            }
            
        }
        strings.join("")
    }

    // pub fn get_contents(&self) {
    //     let content
    // }
}

#[derive(Deserialize, Debug, Clone)]
pub enum PostContentType {
    Paragraph,
    Image,
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