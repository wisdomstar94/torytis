use std::{env, fs::{self}, collections::HashMap};
use commander::functions::run_command::run_command;
use serde_json::Value;

use crate::functions::{build_preprocess::build_preprocess, move_public_to_dot_torytis::move_public_to_dot_torytis, script_bundle::script_bundle, script_postprocess::script_postprocess, skin_html_replace::skin_html_replace};

#[derive(clap::Args)]
#[command(
  about="torytis 프로젝트를 빌드합니다.", 
  long_about = None)
]
pub struct CliArgs {
    /// true : images/ 폴더 없는 구조로 빌드, false : images/ 폴더 구조로 빌드 (기본 값: true)
    #[arg(short='f', long="flat")]
    flat: Option<bool>,
}

pub fn run(args: CliArgs) {
    let flat = args.flat.unwrap_or_else(|| true);

    let working_dir_path_buf = env::current_dir().unwrap();
    let torytis_build_js_file_path_buf = working_dir_path_buf.join("torytis.build.mjs");
    let dot_torytis_index_xml_path_buf = working_dir_path_buf.join(".torytis").join("index.xml");
    let dot_torytis_index_xml_path = dot_torytis_index_xml_path_buf.as_path();

    build_preprocess(&flat);
    
    // src/**/*.script.tsx 파일들을 읽어서 .torytis/script.ts 파일 만들기
    script_bundle();

    // js 를 필요로 하는 로직 실행
    {
        let command = format!("node {}", torytis_build_js_file_path_buf.to_str().unwrap());
        println!("> {}", command);
        let _ = run_command(command.as_str()).unwrap();
        // println!("<- {:?}", output);
    }

    // 현재 시점 .torytis 에 존재하는 파일들
    // style.css
    // script.js
    // script.ts
    // skin.html

    // script.js 파일 내용 수정
    script_postprocess(&flat);

    // skin.html 파일 내용 치환하기
    skin_html_replace(&false);

    // src/public 폴더 밑에 있는 파일들을 모두 .torytis/ 폴더 밑으로 복사하기
    move_public_to_dot_torytis(&flat);

    // .torytis/index.xml 에서 문자 치환하기
    let dot_torytis_index_xml_content = fs::read_to_string(dot_torytis_index_xml_path).unwrap();
    let mut dot_torytis_index_xml_content_new = dot_torytis_index_xml_content.clone();

    let package_json_path_buf = working_dir_path_buf.join("package.json");
    let package_json_path = package_json_path_buf.as_path();
    let package_json_string = fs::read_to_string(package_json_path).unwrap();
    let package_json = serde_json::from_str::<HashMap<String, Value>>(&package_json_string);
    if let Ok(result) = package_json {
        if let Some(version) = result.get("version") {
            dot_torytis_index_xml_content_new = dot_torytis_index_xml_content_new.replace("{ version }", version.as_str().unwrap());
        }
        if let Some(name) = result.get("name") {
            dot_torytis_index_xml_content_new = dot_torytis_index_xml_content_new.replace("{ project_name }", name.as_str().unwrap());
        }
        fs::write(dot_torytis_index_xml_path, &dot_torytis_index_xml_content_new).unwrap();
    }
}