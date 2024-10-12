use regex::Regex;

pub fn replace_skin_html_content(html_string: &String, is_dev: &bool) -> String {
  let mut result = String::from(html_string);
  result = result.replacen("</head>", "<link href=\"./style.css\" as=\"style\" rel=\"preload\" /></head>", 1);
  result = result.replacen("</head>", "<link href=\"./style.css\" type=\"text/css\" rel=\"stylesheet\" /></head>", 1);
  result = result.replacen("</head>", "<script src=\"./images/script.js\"></script></head>", 1);
  if *is_dev {
    result = result.replacen("</head>", r#"<script src="https://cdn.socket.io/4.7.5/socket.io.min.js" integrity="sha384-2huaZvOR9iDzHqslqwpR87isEmrfxqyWOF7hr7BY6KG0+hVKLoEXMPUJw3ynWuhO" crossorigin="anonymous"></script></head>"#, 1);
    result = result.replacen("</head>", r#"<script src="/virtualcdn/socket-dispose.js"></script></head>"#, 1);
  }
  result = result.replacen("<html", "<!DOCTYPE html><html", 1);
  result = result.replace("<tt_html_comment>", "<!-- ");
  result = result.replace("</tt_html_comment>", " -->");
  result = result.replace("<meta charSet", "<meta charset");
  result = result.replace("tt-onclick", "onclick");
  result = result.replace("tt-onmouseover", "onmouseover");
  result = result.replace("tt-onmouseout", "onmouseout");
  result = result.replace("tt-onmouseenter", "onmouseenter");
  result = result.replace("tt-onmouseleave", "onmouseleave");
  result = result.replace("tt-onkeypress", "onkeypress");
  result = result.replace("tt-onkeydown", "onkeydown");
  result = result.replace("tt-value", "value");
  result = result.replace("tt-onload", "onload");
  result = result.replace("tt-onerror", "onerror");

  let pattern = r#"tt-onlyattr=\"(.*?)\""#;
  let re = Regex::new(pattern).unwrap();
  let output_string = re.replace_all(&result.as_str(), r#"$1"#);
  result = output_string.to_string();
  result
}