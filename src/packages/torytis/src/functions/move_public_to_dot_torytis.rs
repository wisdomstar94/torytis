use std::{fs::{self, DirEntry}, io};

use crate::common::{get_src_public_dir_path_buf, get_torytis_dir_path_buf, get_torytis_images_dir_path_buf};

pub fn move_public_to_dot_torytis(flat: &bool) {
  let root_filenames: Vec<&str> = vec!["index.xml", "preview1600.jpg", "preview256.jpg", "preview560.jpg", "skin.html", "style.css"];

  // let working_dir_path_buf = env::current_dir().unwrap();
  let dot_torytis_dir_path_buf = get_torytis_dir_path_buf();

  let src_public_dir_path_buf = get_src_public_dir_path_buf();
  let read_dir_result = fs::read_dir(src_public_dir_path_buf.as_path()).unwrap();
  let read_dir_result_vec: Vec<io::Result<DirEntry>> = read_dir_result.collect();
  for item in read_dir_result_vec {
    let entry = item.unwrap();
    let metadata = entry.metadata().unwrap();
    if metadata.is_dir() {
      continue;
    }
    let entry_path_buf = entry.path();
    let filename = entry_path_buf.as_path().file_name().unwrap().to_str().unwrap();

    let after_path_buf = if !flat && !root_filenames.contains(&filename) {
      get_torytis_images_dir_path_buf().join(filename)
    } else {
      dot_torytis_dir_path_buf.join(filename)
    };

    fs::copy(entry_path_buf.as_path(), after_path_buf).unwrap();
  }
}