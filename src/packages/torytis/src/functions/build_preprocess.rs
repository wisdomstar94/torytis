use std::{env, fs};

pub fn build_preprocess(flat: &bool) {
  let working_dir_path_buf = env::current_dir().unwrap();
  // let torytis_build_js_file_path_buf = working_dir_path_buf.join("torytis.build.mjs");
  // let dot_torytis_index_xml_path_buf = working_dir_path_buf.join(".torytis").join("index.xml");
  // let dot_torytis_index_xml_path = dot_torytis_index_xml_path_buf.as_path();
  let src_public_index_xml_path_buf = working_dir_path_buf.join("src").join("public").join("index.xml");
  let src_public_index_xml_path = src_public_index_xml_path_buf.as_path();

  // src/public/index.xml 체크
  if let Err(_) = fs::metadata(src_public_index_xml_path) {
    panic!("src/public/index.xml 파일이 존재하지 않습니다. 해당 파일을 생성 후 다시 시도해주세요. (https://tistory.github.io/document-tistory-skin/common/index.xml.html)")
  }

  // .torytis 폴더 체크 및 생성
  let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
  let dot_torytis_dir_path = dot_torytis_dir_path_buf.as_path();
  if let Err(_) = fs::metadata(dot_torytis_dir_path) {
    fs::create_dir_all(dot_torytis_dir_path).unwrap();
  } else {
    fs::remove_dir_all(dot_torytis_dir_path).unwrap();
    fs::create_dir_all(dot_torytis_dir_path).unwrap();
  }

  // flat 이 false 일 경우, .torytis/ 폴더 밑에 images/ 폴더 생성
  let dot_torytis_images_dir_path_buf = dot_torytis_dir_path_buf.join("images/");
  let dot_torytis_images_dir_path = dot_torytis_images_dir_path_buf.as_path();
  if !flat {
    if let Err(_) = fs::metadata(dot_torytis_images_dir_path) {
      fs::create_dir_all(dot_torytis_images_dir_path).unwrap();
    } 
  }

  // .torytis/script.ts 생성 (watch 초기화용)
  let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
  let dot_torytis_script_ts_path_buf = dot_torytis_dir_path_buf.join("script.ts");
  fs::write(dot_torytis_script_ts_path_buf.as_path(), "").unwrap();
}