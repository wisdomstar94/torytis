use std::fs;
use crate::common::get_torytis_dir_skin_html_path_buf;
use super::replace_skin_html_content::replace_skin_html_content;

pub fn skin_html_replace(is_dev: &bool) {
  let dot_torytis_skin_html_file_path_buf = get_torytis_dir_skin_html_path_buf();
  let dot_torytis_skin_html_file_path = dot_torytis_skin_html_file_path_buf.as_path();
  let skin_html_string = fs::read_to_string(dot_torytis_skin_html_file_path).unwrap();
  let skin_html_string_convert = replace_skin_html_content(&skin_html_string, is_dev);
  fs::write(dot_torytis_skin_html_file_path, skin_html_string_convert).unwrap();
}