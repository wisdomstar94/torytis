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
        let scripts_block_string = get_scripts_block_string_from_package_json_content(&package_json_content_mut);
        if let Some(string) = scripts_block_string {
            // scripts 블록이 있는 경우
            let mut scripts_block_string_new = String::from(string);
            let pattern = r#"\"scripts\":[^{}]*\{"#;
            let regex = Regex::new(&pattern).unwrap();

            if !scripts_block_string_new.contains("\"tsc\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"scripts\": {\n");
                insert_string.push_str("\t\t\"tsc\": \"tsc\",");
                scripts_block_string_new = regex.replace(&scripts_block_string_new, insert_string).to_string();
            }

            if !scripts_block_string_new.contains("\"tailwindcss\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"scripts\": {\n");
                insert_string.push_str("\t\t\"tailwindcss\": \"tailwindcss\",");
                scripts_block_string_new = regex.replace(&scripts_block_string_new, insert_string).to_string();
            }

            package_json_content_mut = get_scripts_block_regex().replace(&package_json_content_mut, &scripts_block_string_new).to_string();
        } else {
            // scripts 블록이 없는 경우
            panic!("package.json 에 \"scripts\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");
        }

        let dev_dependencies_block_string = get_dev_dependencies_block_string_from_package_json_content(&package_json_content);
        if let Some(string) = dev_dependencies_block_string {
            // devDependencies 블록이 있는 경우
            let mut dev_dependencies_block_string_new = String::from(string);
            let pattern = r#"\"devDependencies\":[^{}]*\{"#;
            let regex = Regex::new(&pattern).unwrap();

            if !dev_dependencies_block_string_new.contains("\"esbuild-sass-plugin\":") {
                let mut insert_string = String::from("");
                insert_string.push_str("\"devDependencies\": {\n");
                insert_string.push_str("\t\t\"esbuild-sass-plugin\": \"^2\",");
                dev_dependencies_block_string_new = regex.replace(&dev_dependencies_block_string_new, insert_string).to_string();
            }

            package_json_content_mut = get_dev_dependencies_regex().replace(&package_json_content_mut, &dev_dependencies_block_string_new).to_string();
        } else {
            // devDependencies 블록이 없는 경우
            panic!("package.json 에 \"devDependencies\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");
        }

        fs::write(package_json_file_path, &package_json_content_mut).unwrap();
    }

    println!("-> torytis 마이그레이션 종료!");
    println!("-> 참고 : package.json 에 새로운 종속성이 추가되었을 경우 'npm install' 명령어를 실행해주세요!");
}

fn get_scripts_block_regex() -> Regex {
    let pattern = r#""scripts":[^{}]*\{[^{}]*\}"#;
    let regex = Regex::new(&pattern).unwrap();
    regex
}

fn get_dev_dependencies_regex() -> Regex {
    let pattern = r#""devDependencies":[^{}]*\{[^{}]*\}"#;
    let regex = Regex::new(&pattern).unwrap();
    regex
}

fn get_scripts_block_string_from_package_json_content(input: &str) -> Option<String> {
    let mut result: Option<String> = None;
    if let Some(captured) = get_scripts_block_regex().captures(input) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(script_block) = captured.get(0) {
            // println!("{}", script_block.as_str());
            result = Some(script_block.as_str().to_string());
        }
    }
    result
}

fn get_dev_dependencies_block_string_from_package_json_content(input: &str) -> Option<String> {
    let mut result: Option<String> = None;
    if let Some(captured) = get_dev_dependencies_regex().captures(input) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(script_block) = captured.get(0) {
            // println!("{}", script_block.as_str());
            result = Some(script_block.as_str().to_string());
        }
    }
    result
}

fn remove_test_tail(version: &str) -> String {
    let pattern = r#"-test[^{}]*"#;
    let regex = Regex::new(&pattern).unwrap();
    regex.replace(version, "").to_string()
}