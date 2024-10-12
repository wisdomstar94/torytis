use std::{env, fs};

pub fn script_postprocess(flat: &bool) {
  let working_dir_path_buf = env::current_dir().unwrap();
  let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
  let script_js_file_path_buf = dot_torytis_dir_path_buf.join("script.js");
  let script_js_file_content = fs::read_to_string(script_js_file_path_buf.clone()).unwrap();
  let mut new_script_js_file_content = String::from("");
  new_script_js_file_content.push_str("(function(){\n");
  new_script_js_file_content.push_str(script_js_file_content.as_str());
  new_script_js_file_content.push_str("\n})();");
  fs::write(script_js_file_path_buf.clone(), new_script_js_file_content).unwrap();
  if !flat {
    let images_script_js_file_path_buf = dot_torytis_dir_path_buf.join("images").join("script.js");
    fs::rename(script_js_file_path_buf.as_path(), images_script_js_file_path_buf.as_path()).unwrap()
  }

  if *flat {
    // script.ts 파일 삭제
    let dot_torytis_script_ts_path_buf = dot_torytis_dir_path_buf.join("script.ts");
    fs::remove_file(dot_torytis_script_ts_path_buf.as_path()).unwrap();
  }
}