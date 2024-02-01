use std::{cell::RefCell, ops::Deref, rc::{Rc, Weak}};
use regex::Regex;

pub struct SelectOptions<'a> {
  pub element_name: &'a str,
  pub attrs: Option<Vec<(&'a str, &'a str)>>,
  pub is_attrs_check_string_contain: bool,
}

pub struct Bucket {
  pub html: Rc<RefCell<String>>,
  pub html_snapshot: Rc<RefCell<String>>,
  pub parent: Weak<Bucket>,
  pub buckets: Rc<RefCell<Option<Vec<Rc<Bucket>>>>>,
  pub buckets_replacer: Rc<RefCell<Option<Box<dyn Fn(String, Option<String>) -> String>>>>,
  pub select_element_name: Rc<RefCell<String>>,
  pub is_commited: Rc<RefCell<bool>>,
  pub chaining_list: Rc<RefCell<Vec<(Vec<Rc<Bucket>>, Box<dyn Fn(String, Option<String>) -> String>)>>>,
}

pub fn select_from_html_string(html: &str, search_options: &SelectOptions) -> Vec<String> {
  let regex = Regex::new(format!(r#"<{}\s?(([^\"<>]*)=\"([^\"]*)\")*>((.|\n)*?)</{}>"#, search_options.element_name, search_options.element_name).as_str()).unwrap();
  let is_attrs_check_string_contain = search_options.is_attrs_check_string_contain;
  let result = regex.find_iter(html).filter(|x| {
    let html_block = x.as_str();
    if let Some(search_attrs) = &search_options.attrs {
      // 속성도 같이 체크해야 할 때
      let element_attrs = get_attr_list_from_html(html_block, search_options.element_name);
      let mut is_exist_matched_attr = false;
      for search_attr in search_attrs {
        for element_attr in &element_attrs {
          if is_attrs_check_string_contain {
            if search_attr.0 == element_attr.0 && element_attr.1.contains(search_attr.1) {
              is_exist_matched_attr = true;
            }
          } else {
            if search_attr.0 == element_attr.0 && element_attr.1 == search_attr.1 {
              is_exist_matched_attr = true;
            }
          }
        }
      
      }
      is_exist_matched_attr
    } else {
      // 요소만 체크해도 될 때
      true
    }
  }).map(|x| x.as_str().to_owned());
  result.collect::<Vec<String>>()
}

pub fn select_from_html_string_one(html: &str, search_options: &SelectOptions) -> Option<String> {
  let mut result: Option<String> = None;
  let results = select_from_html_string(html, search_options);
  if let Some(k) = results.first() {
    result = Some(k.to_owned());
  }
  result
}

pub fn html_string_root_element_unwrap(html: &str, root_element_name: &str) -> String {
  let mut convert_string = html.to_owned();

  let pattern1_format = format!(r#"<{}\s?(([^\"<>]*)=\"([^\"]*)\")*>"#, root_element_name);
  let pattern1 = pattern1_format.as_str();
  let regex1 = Regex::new(pattern1).unwrap();

  let pattern2_format = format!(r#"</{}>"#, root_element_name);
  let pattern2 = pattern2_format.as_str();

  convert_string = regex1.replacen(&convert_string, 1, "").to_string();
  convert_string = convert_string.replacen(pattern2, "", 1);

  convert_string
}

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
                //   println!("char {}", item);
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

impl Bucket {
  pub fn new(html: &str) -> Rc<Self> {
    let r = Rc::new(
      Bucket {
        html: Rc::new(RefCell::new(html.to_owned())),
        html_snapshot: Rc::new(RefCell::new(html.to_owned())),
        parent: Weak::new(),
        buckets: Rc::new(RefCell::new(None)),
        buckets_replacer: Rc::new(RefCell::new(None)),
        select_element_name: Rc::new(RefCell::new(String::from("root"))),
        is_commited: Rc::new(RefCell::new(false)),
        chaining_list: Rc::new(RefCell::new(vec![])),
      } 
    );
    r
  }

  pub fn select(&self, search_options: SelectOptions) -> &Self {
    if self.chaining_list.deref().borrow().len() == 0 {
      let binding3 = self.html.deref().borrow();
      let target_html = binding3.as_str();
      let buckets = select_from_html_string(target_html, &search_options).iter().map(|x| -> Rc<Bucket> {
        Rc::new(
          Bucket {
            html: Rc::new(RefCell::new(x.as_str().to_owned())),
            html_snapshot: Rc::new(RefCell::new(x.as_str().to_owned())),
            parent: Weak::new(),
            buckets: Rc::new(RefCell::new(None)),
            buckets_replacer: Rc::new(RefCell::new(None)),
            select_element_name: Rc::new(RefCell::new(String::from(search_options.element_name))),
            is_commited: Rc::new(RefCell::new(false)),
            chaining_list: Rc::new(RefCell::new(vec![])),
          }
        )
      }).collect::<Vec<Rc<Bucket>>>();
      *self.buckets.deref().borrow_mut() = Some(buckets);
    } else {
      let binding = self.chaining_list.deref().borrow();
      let last_child = binding.last().unwrap();
      
      let parent_buckets = &last_child.0;
      
      let mut total_vec_rc_bucket: Vec<Rc<Bucket>> = vec![];
      for item in parent_buckets {
        let target_html = item.html.deref().borrow().to_owned();
        let buckets = select_from_html_string(&target_html, &search_options).iter().map(|x| -> Rc<Bucket> {
          Rc::new(
            Bucket {
              html: Rc::new(RefCell::new(x.as_str().to_owned())),
              html_snapshot: Rc::new(RefCell::new(x.as_str().to_owned())),
              parent: Rc::downgrade(&Rc::clone(item)),
              buckets: Rc::new(RefCell::new(None)),
              buckets_replacer: Rc::new(RefCell::new(None)),
              select_element_name: Rc::new(RefCell::new(String::from(search_options.element_name))),
              is_commited: Rc::new(RefCell::new(false)),
              chaining_list: Rc::new(RefCell::new(vec![])),
            }
          )
        }).collect::<Vec<Rc<Bucket>>>();
        for b in buckets {
          total_vec_rc_bucket.push(b);
        }
      }
      *self.buckets.deref().borrow_mut() = Some(total_vec_rc_bucket);
    }
    &self
  }

  pub fn replacer(&self, f: impl Fn(String, Option<String>) -> String + 'static) -> &Self {
    *self.buckets_replacer.deref().borrow_mut() = Some(Box::new(f));
    &self
  }

  pub fn chain(&self) -> &Self {
    let b = self.buckets.take();
    let buckets = b.unwrap(); // temp

    let p = self.buckets_replacer.take();
    let replacer = p.unwrap(); // temp

    let mut k = self.chaining_list.borrow_mut();
    k.push((
      buckets,
      replacer,
    ));

    &self
  }

  pub fn commit(&self) {
    let mut l = self.chaining_list.take();
    l.reverse();
    for item in l {
      let childs = item.0;
      let callback = item.1;

      for self2 in childs {
        let bbb = &self2.select_element_name;
        let select_element_name = bbb.deref().borrow().to_owned();

        let current = self2.parent.upgrade();
        let parent: &Bucket = if let Some(ref v) = current {
          v.as_ref()
        } else {
          &self
        };

        
        let mut parent_html_borrow_mut = parent.html.deref().borrow_mut();
      
        let mut self_html_borrow_mut = self2.html.deref().borrow_mut();
        let mut self_html_snapshot_borrow_mut = self2.html_snapshot.deref().borrow_mut();

        let mut replace_from_html = self_html_snapshot_borrow_mut.as_str();

        let changed_html = self_html_borrow_mut.as_str().to_owned();
        let parent_html = parent_html_borrow_mut.as_str().to_owned();

        let is_matched = parent_html.matches(replace_from_html).count() >= 1;
        if !is_matched {
          replace_from_html = &changed_html;
        }

        let mut unwrap_replace_from_html: Option<String> = None;
        if select_element_name != "root" {
          // println!("select_element_name!! {}", select_element_name);
          unwrap_replace_from_html = Some(html_string_root_element_unwrap(&changed_html, &select_element_name));
        }
        // println!("is_matched : {}", is_matched);
        let to = callback(changed_html.to_string(), unwrap_replace_from_html);
        println!("to : {}", to);
        *self_html_borrow_mut = to.to_string();
        let result = parent_html.replace(replace_from_html, to.as_str());
        *parent_html_borrow_mut = result.clone();

        *self_html_snapshot_borrow_mut = to.to_string();
        
      }
    }
  }

  pub fn get_html(&self) -> String {
    let binding = self.html.deref().borrow();
    binding.to_owned()
  }
}
