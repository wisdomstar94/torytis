use std::{env, path::PathBuf, fs};

use chrono::NaiveDateTime;
use include_dir::Dir;

use crate::statics::STATIC_DIR;

pub fn get_working_dir_path_buf() -> PathBuf {
    env::current_dir().unwrap()
}

pub fn get_torytis_dir_path_buf() -> PathBuf {
    get_working_dir_path_buf().join(".torytis")
}

pub fn get_torytis_dir_skin_html_path_buf() -> PathBuf {
    get_torytis_dir_path_buf().join("skin.html")
}

pub fn get_torytis_dev_config_json_path_buf() -> PathBuf {
    get_working_dir_path_buf().join("torytis-dev.config.json")
}

pub fn get_index_xml_path_buf() -> PathBuf {
    get_working_dir_path_buf().join("src").join("public").join("index.xml")
}

pub fn get_temp_html_path_buf() -> PathBuf {
    get_working_dir_path_buf().join("bin").join("skin.html")
}

pub fn get_skin_html_content() -> String {
    let path_buf = get_torytis_dir_skin_html_path_buf();
    fs::read_to_string(path_buf.as_path()).unwrap()
}

pub fn get_torytis_dev_config_json_content() -> String {
    let path_buf = get_torytis_dev_config_json_path_buf();
    fs::read_to_string(path_buf.as_path()).unwrap()
}

pub fn get_index_xml_content() -> String {
    let path_buf = get_index_xml_path_buf();
    fs::read_to_string(path_buf.as_path()).unwrap()
}

pub fn get_temp_html_content() -> String {
    let path_buf = get_temp_html_path_buf();
    fs::read_to_string(path_buf.as_path()).unwrap()
}

pub fn get_static_tistory_cdn_dir_path_buf() -> &'static Dir<'static> {
    let dir = STATIC_DIR.get_dir("tistory-cdn").unwrap();
    dir
}

pub struct PaginationCalculateInfo {
    pub max_page_num: u32,
    pub best_left_num: Option<u32>,
    pub center_page_num_list: Vec<u32>,
    pub best_right_num: Option<u32>,
}

pub fn get_pagination_calculate(total_count: usize, page: u32, size: u32) -> PaginationCalculateInfo {
    let max_page_num: f32 = ((total_count as f32 / size as f32)) as f32;
    let max_page_num = max_page_num.ceil() as u32;

    let mut best_left_num: Option<u32> = None;
    let mut center_left_vec: Vec<u32> = Vec::new();
    let center_center: u32 = page as u32;
    let mut center_right_vec: Vec<u32> = Vec::new();
    let mut best_right_num: Option<u32> = None;

    let temp: i32 = center_center as i32 - 3;
    if temp <= 2 {
        let mut current_page: i32 = center_center.clone() as i32 - 1 as i32;
        loop {
            if current_page <= 0 {
                break;
            }
            center_left_vec.push(current_page as u32);
            current_page -= 1;
        }
        center_left_vec.reverse();
    } else {
        let temp: Vec<i32> = vec![center_center as i32 - 3, center_center as i32 - 2, center_center as i32 - 1];
        for index in temp {
            if index <= 0 {
                continue;
            }
            center_left_vec.push(index as u32);
        }
        best_left_num = Some(1);
    }

    if center_center + 4 >= max_page_num {
        let mut current_page = &center_center + 1;
        loop {
            if current_page > max_page_num {
                break;
            }
            center_right_vec.push(current_page);
            current_page += 1;
        }
    } else {
        let temp: Vec<u32> = vec![center_center + 1, center_center + 2, center_center + 3];
        for index in temp {
            center_right_vec.push(index);
        }
        best_right_num = Some(max_page_num);
    }

    let center_page_num_list: Vec<u32> = [center_left_vec, vec![center_center], center_right_vec].concat();

    PaginationCalculateInfo {
        max_page_num,
        best_left_num,
        center_page_num_list,
        best_right_num,
    }
}

pub fn date_format(datetime_str: &str, format: &str) -> String {
    let time = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").unwrap().format(format).to_string();
    time
}