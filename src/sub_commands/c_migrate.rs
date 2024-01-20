use std::{env, fs::{self}, collections::HashMap};
use crate::statics::{STATIC_DIR, VERSION};
use regex::Regex;
use serde_json::Value;
use version_compare::Version;

#[derive(clap::Args)]
#[command(
  about="torytis 프로젝트를 마이그레이션합니다.", 
  long_about = None)
]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    let version = remove_test_tail(&VERSION);
    let working_dir_path_buf = env::current_dir().unwrap();

    // ## package.json 파일 체크
    let package_json_file_path_buf = working_dir_path_buf.join("package.json");
    let package_json_file_path = package_json_file_path_buf.as_path();
    if let Err(_) = fs::metadata(package_json_file_path) {
        panic!("-> npm 프로젝트 폴더가 아닙니다.");
    }
    
    let package_json_content = fs::read_to_string(package_json_file_path).unwrap();
    let mut package_json_content_mut = package_json_content.clone();
    // println!("-> package_json_content : {:#?}", package_json_content);
    let json_parse_result = serde_json::from_str::<HashMap<String, Value>>(&package_json_content_mut);
    if let Err(err) = json_parse_result {
        panic!("-> package.json 내용에 오류가 있습니다. : {:#?}", err);
    }

    println!("-> torytis 마이그레이션 시작!");
    println!("-> 현재 torytis 버전 : {:#?}", VERSION);
    
    // v0 -> v1 
    if Version::from("1.0.0").unwrap() <= Version::from(version.as_str()).unwrap() {
    // if true {
        // torytis-build.tsx 파일 체크
        let torytis_build_tsx_file_path_buf = working_dir_path_buf.join("torytis-build.tsx");
        let torytis_build_tsx_file_path = torytis_build_tsx_file_path_buf.as_path();
        if let Err(_) = fs::metadata(torytis_build_tsx_file_path) {
            let file = STATIC_DIR.get_file("project-template/torytis-build.tsx").unwrap();
            let file_content = file.contents_utf8().unwrap();
            fs::write(torytis_build_tsx_file_path, file_content).unwrap();
        }
        // package.json 파일 체크
        package_json_content_mut = apply_scripts_block(&package_json_content_mut);
        package_json_content_mut = apply_dev_dependencies_block(&package_json_content_mut);
        fs::write(package_json_file_path, &package_json_content_mut).unwrap();
        // .gitignore 파일 체크
        let gitignore_file_path_buf = working_dir_path_buf.join(".gitignore");
        let gitignore_file_path = gitignore_file_path_buf.as_path();
        if let Ok(_) = fs::metadata(gitignore_file_path) {
            let mut gitignore_file_content = fs::read_to_string(gitignore_file_path).unwrap();
            if !gitignore_file_content.contains("torytis-build.js") {
                gitignore_file_content.push_str("\ntorytis-build.js");
            }
            fs::write(gitignore_file_path, gitignore_file_content).unwrap();
        }
    }

    println!("-> torytis 마이그레이션 종료!");
    println!("-> 참고 : package.json 에 새로운 종속성이 추가되었을 경우 'npm install' 명령어를 실행해주세요!");
}

fn remove_test_tail(version: &str) -> String {
    let pattern = r#"-test[^{}]*"#;
    let regex = Regex::new(&pattern).unwrap();
    regex.replace(version, "").to_string()
}

fn apply_scripts_block(package_json_content: &str) -> String {
    let mut result = String::from(package_json_content);
    let pattern = r#""scripts":[^{}]*\{[^{}]*\}"#;
    let regex = Regex::new(&pattern).unwrap(); // "scripts": { .. }  <-- scripts 전체 블록을 선택
    if let Some(captured) = regex.captures(package_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(script_block) = captured.get(0) {
            // scripts 블록이 있는 경우
            let script_block_string: String = script_block.as_str().to_string();
            let mut scripts_block_string_new = String::from(script_block_string);
            let pattern2 = r#"\"scripts\":[^{}]*\{"#;
            let regex2 = Regex::new(&pattern2).unwrap(); // "scripts": {  <-- 이 한줄을 선택

            // "build:variable" 이 이미 있는 경우 수정
            if scripts_block_string_new.contains("\"build:variable\":") {
                let pattern3 = r#"\"build:variable\"[^{}]*:[^{}]*\"[^{}]*\""#; // "build:variable": ".."  <-- 이 한줄을 선택
                let regex3 = Regex::new(&pattern3).unwrap();
                scripts_block_string_new = regex3.replace(&scripts_block_string_new, "\"build:variable\": \"torytis varbuild\"").to_string();
            }

            // "tsc" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"tsc\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"scripts\": {\n");
                insert_string.push_str("\t\t\"tsc\": \"tsc\",");
                scripts_block_string_new = regex2.replace(&scripts_block_string_new, insert_string).to_string();
            }

            // "tailwindcss" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"tailwindcss\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"scripts\": {\n");
                insert_string.push_str("\t\t\"tailwindcss\": \"tailwindcss\",");
                scripts_block_string_new = regex2.replace(&scripts_block_string_new, insert_string).to_string();
            }

            // "torytis" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"torytis\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"scripts\": {\n");
                insert_string.push_str("\t\t\"torytis\": \"torytis\",");
                scripts_block_string_new = regex2.replace(&scripts_block_string_new, insert_string).to_string();
            }

            result = regex.replace(&result, &scripts_block_string_new).to_string();
        } else {
            // scripts 블록이 없는 경우
            panic!("package.json 에 \"scripts\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");   
        }
    }
    result
}

fn apply_dev_dependencies_block(package_json_content: &str) -> String {
    let mut result: String = String::from(package_json_content);
    let pattern = r#""devDependencies":[^{}]*\{[^{}]*\}"#;
    let regex = Regex::new(&pattern).unwrap();
    if let Some(captured) = regex.captures(package_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(script_block) = captured.get(0) {
            // devDependencies 블록이 있는 경우
            let script_block_string: String = script_block.as_str().to_string();
            let mut dev_dependencies_block_string_new = String::from(script_block_string);
            let pattern2 = r#"\"devDependencies\":[^{}]*\{"#;
            let regex2 = Regex::new(&pattern2).unwrap();

            if !dev_dependencies_block_string_new.contains("\"esbuild-sass-plugin\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"devDependencies\": {\n");
                insert_string.push_str("\t\t\"esbuild-sass-plugin\": \"^2\",");
                dev_dependencies_block_string_new = regex2.replace(&dev_dependencies_block_string_new, insert_string).to_string();
            }

            result = regex.replace(&result, &dev_dependencies_block_string_new).to_string();
        } else {
            // devDependencies 블록이 없는 경우
            panic!("package.json 에 \"devDependencies\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");   
        }
    }
    result
}