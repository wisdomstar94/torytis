use std::{cell::RefCell, fs, path::PathBuf};

pub struct FileContentController {
  file_path_buf: PathBuf,
  file_content: RefCell<String>,
}

impl FileContentController {
  pub fn new(file_path_buf: PathBuf) -> Self {
    let file_content = fs::read_to_string(file_path_buf.as_path()).unwrap();

    Self {
      file_path_buf,
      file_content: RefCell::new(file_content),
    }
  }

  pub fn change(&self, f: impl Fn(&String) -> String) -> &Self {
    let mut file_content = self.file_content.borrow_mut();
    let file_content_prev = file_content.to_owned();

    *file_content = f(&file_content_prev);
    self
  }

  pub fn commit(&self) {
    let value = &self.file_content.borrow().to_owned();
    fs::write(&self.file_path_buf, value).unwrap();
  }
}