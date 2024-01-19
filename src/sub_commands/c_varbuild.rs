use std::{env, fs::{self}};
use regex::Regex;
use xmltree::Element;

use crate::statics::STATIC_DIR;

#[derive(clap::Args)]
#[command(
    about="src/public/index.xml 에 작성된 variables 내용을 파싱하여 d.ts 파일을 생성합니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    let working_dir_path_buf = env::current_dir().unwrap();
    let src_public_index_xml_path_buf = working_dir_path_buf.join("src").join("public").join("index.xml");
    let src_public_index_xml_path = src_public_index_xml_path_buf.as_path();
    let content = fs::read_to_string(src_public_index_xml_path).unwrap();

    let element = Element::parse(content.as_bytes()).unwrap();
    let variables = element.get_child("variables").unwrap();

    let file = STATIC_DIR.get_file("torytis-variable.d.ts").unwrap();
    let file_content = file.contents_utf8().unwrap();

    let mut declaration_text: String = String::from("");

    let variables_list = &variables.children;
    for variable_group in variables_list {
        if let Some(variable_group_element) = variable_group.as_element() {
            let variable = &variable_group_element.children;
            for item in variable {
                if let Some(variable_element) = item.as_element() {
                    // name 가져오기
                    if let Some(name_element) = variable_element.get_child("name") {
                        if let Some(text) = name_element.get_text() {
                            declaration_text.push_str(format!("\t\t's_if_var_{}': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;\n", text).as_str());
                            declaration_text.push_str(format!("\t\t's_not_var_{}': React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;\n", text).as_str());
                        }
                    }
                }
            }
        }
    }

    let torytis_variable_d_ts_file_path_buf = working_dir_path_buf.join("torytis-variable.d.ts");
    let torytis_variable_d_ts_file_path = torytis_variable_d_ts_file_path_buf.as_path();
    let pattern = r"\n\n\s+}";
    let regex = Regex::new(&pattern).unwrap();
    let result = file_content.replace("//THIS_IS_REPLACE_SPOT//", declaration_text.as_str());
    let result = regex.replace_all(&result, "\n\t}").to_string();

    fs::write(torytis_variable_d_ts_file_path, result).unwrap();
}