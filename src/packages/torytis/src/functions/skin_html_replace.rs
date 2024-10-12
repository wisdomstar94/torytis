use std::{env, fs};
use super::replace_skin_html_content::replace_skin_html_content;

pub fn skin_html_replace(is_dev: &bool) {
  let working_dir_path_buf = env::current_dir().unwrap();
  let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
  let dot_torytis_skin_html_file_path_buf = dot_torytis_dir_path_buf.join("skin.html");
  let dot_torytis_skin_html_file_path = dot_torytis_skin_html_file_path_buf.as_path();
  let skin_html_string = fs::read_to_string(dot_torytis_skin_html_file_path).unwrap();
  let skin_html_string_convert = replace_skin_html_content(&skin_html_string, is_dev);
  fs::write(dot_torytis_skin_html_file_path, skin_html_string_convert).unwrap();
}