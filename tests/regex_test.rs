use std::ops::Deref;
use html_regex::{html_string_root_element_unwrap, select_from_html_string_one, Bucket, SelectOptions};
use regex::Regex;
use torytis::common::get_temp_html_content;

#[test]
fn regex_html_tag_select_test() {
    let html = get_temp_html_content();
    // let regex = Regex::new(r#"<s_cover\s?(([^\"<>]*)=\"([^\"]*)\")*>(.*?)</s_cover>"#).unwrap();
    let regex = Regex::new(r#"<s_sidebar\s?(([^\"<>]*)=\"([^\"]*)\")*>(.*?)</s_sidebar>"#).unwrap();
    for item in regex.find_iter(&html) {
        println!("ooo {}", item.as_str());
        println!("");
    }
}

#[test]
fn regex_html_attr_parse_test() {
    fn get_attr_list_from_html<'a>(target_html_block: &'a str, element_name: &'a str) -> Vec<(String, String)> {
        let mut vec: Vec<(String, String)> = Vec::new();
        if let Some(v) = Regex::new(format!(r#"<{}\s?(([^\"<>]*)=\"([^\"]*)\")*>"#, element_name).as_str()).unwrap().find(target_html_block) {
            let matched_str = v.as_str();
            for item in Regex::new(format!(r#"[^(<{}\s?)](([^\"<>]*)=\"([^\"]*)\")"#, element_name).as_str()).unwrap().find_iter(matched_str) {
                let item_str = item.as_str();
                let item_str_convert = item_str.replacen("=", "@@@_@@@", 1);
                let item_split = item_str_convert.split("@@@_@@@");
                let item_split_vec = item_split.collect::<Vec<&str>>();
                let attr_name = item_split_vec.get(0).unwrap().trim();
                let attr_value = item_split_vec.get(1).unwrap().trim();
                let mut attr_real_value = String::new();
                let mut index: usize = 0;
                let chars = attr_value.chars().collect::<Vec<char>>();
                let chars_len = chars.len();
                for item in chars {
                    if index == 0 || index == chars_len - 1 {
                        println!("char {}", item);
                        index = index + 1;
                        continue;
                    }
                    attr_real_value.push(item);
                    index = index + 1;
                }
                vec.push((attr_name.to_owned(), attr_real_value.to_owned()));
            }
        }
        vec
    }

    let html = r#"<s_cover name="이름입니다." data-value="값입니다."><div></div></s_cover>"#;
    let vec = get_attr_list_from_html(html, "s_cover");
    println!("vec: {:#?}", vec);
}

#[test]
fn html_regex_bucket_test() {
    // let html = get_temp_html_content();
    
    // let root = Bucket::new(&html);
    // let children1 = root.select(root.clone(), SelectOptions {
    //     element_name: "s_sidebar_element",
    //     attrs: None,
    //     is_attrs_check_string_contain: true,
    // }).replacer(|_, unwrap_str| {
    //     // let mut string = str;
    //     // string = string.replace("<s_sidebar_element>").replace("from", to);
    //     unwrap_str.unwrap()
    // }).get_selected_buckets();
    // for child1 in children1.unwrap() {
    //     child1.commit();
    // }

    // println!("html -> {}", root.get_html());
}

#[test]
fn test1() {
    let html = get_temp_html_content();
    let pattern = format!(r#"<{}\s?(([^\"<>]*)=\"([^\"]*)\")*>((.|\n)*?)</{}>"#, "s_cover_group", "s_cover_group");
    println!("pattern : {}", pattern);
    let regex = Regex::new(pattern.as_str()).unwrap();
    println!("is match? : {}", regex.is_match(&html));
}

#[test]
fn test2() {
    let html = get_temp_html_content();
    let pattern1_format = format!(r#"<{}\s?(([^\"<>]*)=\"([^\"]*)\")*>"#, "s_cover_group");
    let pattern1 = pattern1_format.as_str();
    let regex1 = Regex::new(pattern1).unwrap();

    println!("is match??? {}", regex1.is_match(&html));
    let count = regex1.find_iter(&html).count();
    println!("count {}", count);
}

#[test]
fn html_regex_bucket_test_2() {
    // let html = get_temp_html_content();
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title></title>
            </head>
            <body>
                <s3>
                    <div>
                        <s_cover_group>
                            <section>
                                <div>
                                    <s_cover_rep>
                                        <s_cover name="list">
                                            <!-- 1.1 -->
                                            <div class="list">
                                                <s_cover_item_thumbnail>
                                                    list 썸네일입니다..
                                                </s_cover_item_thumbnail>
                                            </div>
                                        </s_cover>
                                        <s_cover name="list-half">
                                            <!-- 1.2 -->
                                            <div class="list-half">
                                                <s_cover_item_thumbnail>
                                                    list-half 썸네일입니다..
                                                </s_cover_item_thumbnail>
                                            </div>
                                        </s_cover>
                                    </s_cover_rep>
                                </div>
                                <div>
                                    <s_cover_rep>
                                        <s_cover name="list">
                                            <!-- 2.1 -->
                                            <div class="list">
                                                <s_cover_item_thumbnail>
                                                    22 list 썸네일입니다..
                                                </s_cover_item_thumbnail>
                                            </div>
                                        </s_cover>
                                        <s_cover name="list-half">
                                            <!-- 2.2 -->
                                            <div class="list-half">
                                                <s_cover_item_thumbnail>
                                                    22 list-half 썸네일입니다..
                                                </s_cover_item_thumbnail>
                                            </div>
                                        </s_cover>
                                    </s_cover_rep>
                                </div>
                            </section>
                        </s_cover_group>
                    </div>
                </s3>
            </body>
        </html>
    "#;

    let root = Bucket::new(&html);
    root.select(&root, SelectOptions {
        element_name: "s_cover_group",
        attrs: None,
        is_attrs_check_string_contain: true,
    });
    root.replacer(|_, unwrap_str| {
        let result = unwrap_str.unwrap();
        result
    });
    for child1 in root.buckets.deref().borrow_mut().take().unwrap() {
        // child1.commit();
        println!("???");
        child1.select(&child1, SelectOptions {
            element_name: "s_cover_rep",
            attrs: None,
            is_attrs_check_string_contain: true,
        });
        child1.replacer(|_, unwrap_str| {
            let result = unwrap_str.unwrap();

            let mut vec: Vec<String> = Vec::new();
            for item in vec!["list", "list-half", "list-half"] {
                let mini_root_html = select_from_html_string_one(&result, &SelectOptions {
                    element_name: "s_cover",
                    attrs: Some(vec![("name", item)]),
                    is_attrs_check_string_contain: true,
                }).unwrap();
                let mini_root = Bucket::new(&mini_root_html);
                mini_root.select(&mini_root, SelectOptions {
                    element_name: "s_cover_item_thumbnail",
                    attrs: None,
                    is_attrs_check_string_contain: true,
                });
                mini_root.replacer(|_, s2| {
                    s2.unwrap()
                });
                for mini_child in mini_root.buckets.deref().borrow_mut().take().unwrap() {
                    mini_child.commit();
                }
                vec.push(html_string_root_element_unwrap(&mini_root.get_html(), "s_cover"));
            }
            vec.join("")
        });
        for child2 in child1.buckets.deref().borrow_mut().take().unwrap() {
            child2.commit();
        }
    }

    println!("html {}", root.get_html());
}