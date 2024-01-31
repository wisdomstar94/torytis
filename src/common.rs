use std::{env, path::PathBuf, fs};

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
