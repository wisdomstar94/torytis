use std::fs;
use crate::common::{get_index_xml_path_buf, get_torytis_dir_path_buf, get_torytis_images_dir_path_buf};
use super::script_bundle::script_bundle;

pub fn build_preprocess(flat: &bool) {
  let src_public_index_xml_path = get_index_xml_path_buf();

  // src/public/index.xml 체크
  if let Err(_) = fs::metadata(src_public_index_xml_path.as_path()) {
    panic!("src/public/index.xml 파일이 존재하지 않습니다. 해당 파일을 생성 후 다시 시도해주세요. (https://tistory.github.io/document-tistory-skin/common/index.xml.html)")
  }

  // .torytis 폴더 체크 및 생성
  let dot_torytis_dir_path_buf = get_torytis_dir_path_buf();
  if let Err(_) = fs::metadata(dot_torytis_dir_path_buf.as_path()) {
    fs::create_dir_all(dot_torytis_dir_path_buf.as_path()).unwrap();
  } else {
    fs::remove_dir_all(dot_torytis_dir_path_buf.as_path()).unwrap();
    fs::create_dir_all(dot_torytis_dir_path_buf.as_path()).unwrap();
  }

  // flat 이 false 일 경우, .torytis/ 폴더 밑에 images/ 폴더 생성
  let dot_torytis_images_dir_path_buf = get_torytis_images_dir_path_buf();
  if !flat {
    if let Err(_) = fs::metadata(dot_torytis_images_dir_path_buf.as_path()) {
      fs::create_dir_all(dot_torytis_images_dir_path_buf.as_path()).unwrap();
    } 
  }

  // .torytis/script.ts 생성 
  script_bundle();
}