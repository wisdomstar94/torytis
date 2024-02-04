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
        // torytis-build.tsx 파일 삭제
        let torytis_build_tsx_file_path_buf = working_dir_path_buf.join("torytis-build.tsx");
        let torytis_build_tsx_file_path = torytis_build_tsx_file_path_buf.as_path();
        if let Ok(_) = fs::metadata(torytis_build_tsx_file_path) {
            fs::remove_file(torytis_build_tsx_file_path).unwrap();
        }
        // torytis-build.js 파일 체크
        let torytis_build_js_file_path_buf = working_dir_path_buf.join("torytis-build.js");
        let torytis_build_js_file_path = torytis_build_js_file_path_buf.as_path();
        if let Err(_) = fs::metadata(torytis_build_js_file_path) {
            let file = STATIC_DIR.get_file("project-template/torytis-build.js").unwrap();
            let file_content = file.contents_utf8().unwrap();
            fs::write(torytis_build_js_file_path, file_content).unwrap();
        } else {
            let file_content = fs::read_to_string(torytis_build_js_file_path).unwrap();
            let mut file_content_mut = file_content.clone();
            file_content_mut = apply_torytis_build_js(&file_content_mut);
            fs::write(torytis_build_js_file_path, file_content_mut).unwrap();
        }
        // package.json 파일 체크
        package_json_content_mut = apply_scripts_block(&package_json_content_mut);
        package_json_content_mut = apply_dev_dependencies_block(&package_json_content_mut);
        fs::write(package_json_file_path, &package_json_content_mut).unwrap();
        // .gitignore 파일 체크
        let gitignore_file_path_buf = working_dir_path_buf.join(".gitignore");
        let gitignore_file_path = gitignore_file_path_buf.as_path();
        if let Ok(_) = fs::metadata(gitignore_file_path) {
            let gitignore_file_content = fs::read_to_string(gitignore_file_path).unwrap();
            let gitignore_file_content_mut = apply_gitignore(&gitignore_file_content);
            fs::write(gitignore_file_path, gitignore_file_content_mut).unwrap();
        }
        // tsconfig.json 파일 체크
        let tsconfig_json_file_path_buf = working_dir_path_buf.join("tsconfig.json");
        let tsconfig_json_file_path = tsconfig_json_file_path_buf.as_path();
        if let Ok(_) = fs::metadata(tsconfig_json_file_path) {
            let tsconfig_json_content = fs::read_to_string(tsconfig_json_file_path).unwrap();
            let mut tsconfig_json_content_mut = tsconfig_json_content.clone();
            tsconfig_json_content_mut = apply_tsconfig_compiler_options_block(&tsconfig_json_content_mut);
            tsconfig_json_content_mut = apply_tsconfig_include_block(&tsconfig_json_content_mut);
            fs::write(tsconfig_json_file_path, tsconfig_json_content_mut).unwrap();
        }
        // tailwind.config.ts 파일 체크
        let tailwind_config_ts_file_path_buf = working_dir_path_buf.join("tailwind.config.ts");
        let tailwind_config_ts_file_path = tailwind_config_ts_file_path_buf.as_path();
        if let Ok(_) = fs::metadata(tailwind_config_ts_file_path) {
            let file_content = fs::read_to_string(tailwind_config_ts_file_path).unwrap();
            let mut file_content_mut = file_content.clone();
            file_content_mut = apply_tailwind_config_ts_content_block(&file_content_mut);
            fs::write(tailwind_config_ts_file_path, file_content_mut).unwrap();
        }
        // torytis-dev.config.json 파일 체크
        let torytis_dev_config_json_file_path_buf = working_dir_path_buf.join("torytis-dev.config.json");
        let torytis_dev_config_json_file_path = torytis_dev_config_json_file_path_buf.as_path();
        if let Err(_) = fs::metadata(torytis_dev_config_json_file_path) {
            let file = STATIC_DIR.get_file("project-template/torytis-dev.config.json").unwrap();
            let file_content = file.contents_utf8().unwrap();
            fs::write(torytis_dev_config_json_file_path, file_content).unwrap();
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
            let mut should_append_items: Vec<String> = Vec::new();

            let script_block_string: String = script_block.as_str().to_string();
            let mut scripts_block_string_new = String::from(script_block_string);
            let pattern2 = r#"\"scripts\":[^{}]*\{"#;
            let regex2 = Regex::new(&pattern2).unwrap(); // "scripts": {  <-- 이 한줄을 선택

            // "build:variable" 이 이미 있는 경우 수정
            if scripts_block_string_new.contains("\"build:variable\":") {
                let pattern3 = r#""build:variable":[^{}\,]*"[^{}\,]*""#; // "build:variable": ".."  <-- 이 한줄을 선택
                let regex3 = Regex::new(&pattern3).unwrap();
                scripts_block_string_new = regex3.replace(&scripts_block_string_new, "\"build:variable\": \"torytis varbuild\"").to_string();
            }

            // "tsc" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"tsc\":") {
                should_append_items.push("\"tsc\": \"tsc\"".to_string());
            }

            // "tailwindcss" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"tailwindcss\":") {
                should_append_items.push("\"tailwindcss\": \"tailwindcss\"".to_string());
            }

            // "torytis" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"torytis\":") {
                should_append_items.push("\"torytis\": \"torytis\"".to_string());
            }

            // "dev" 가 없는 경우 추가
            if !scripts_block_string_new.contains("\"dev\":") {
                should_append_items.push("\"dev\": \"torytis dev\"".to_string());
            }

            if should_append_items.len() > 0 {
                let mut insert_string = format!("\"scripts\": {{\n\t\t{}", should_append_items.join(",\n\t\t")); 
                if is_exist_package_json_scripts_item(&package_json_content) {
                    insert_string.push_str(",");
                }
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

fn apply_tsconfig_compiler_options_block(tsconfig_json_content: &str) -> String {
    let mut result: String = String::from(tsconfig_json_content);
    let pattern = r#""compilerOptions":[^{}]*\{[^{}]*\}"#;
    let regex = Regex::new(&pattern).unwrap();
    
    if let Some(captured) = regex.captures(tsconfig_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(text) = captured.get(0) {
            // compilerOptions 블록이 있는 경우
            let mut should_append_compiler_options_items: Vec<String> = Vec::new();

            let block_string: String = text.as_str().to_string();
            let mut block_string_new = String::from(block_string);
            let pattern2 = r#"\"compilerOptions\":[^{}]*\{"#;
            let regex2 = Regex::new(&pattern2).unwrap();

            if !block_string_new.contains("\"jsx\":") {
                should_append_compiler_options_items.push(r#""jsx": "react-jsx""#.to_string());
            } else {
                block_string_new = Regex::new(r#""jsx": "([^"]*)""#).unwrap().replace(&block_string_new, "\"jsx\": \"react-jsx\"").to_string();
            }

            if !block_string_new.contains("\"allowSyntheticDefaultImports\"") {
                should_append_compiler_options_items.push(r#""allowSyntheticDefaultImports": true"#.to_string());   
            }

            if !block_string_new.contains("\"strict\"") {
                should_append_compiler_options_items.push(r#""strict": true"#.to_string());   
            }

            if should_append_compiler_options_items.len() > 0 {
                let mut insert_string = format!("\"compilerOptions\": {{\n\t\t{}", should_append_compiler_options_items.join(",\n\t\t")); 
                if is_exist_tsconfig_compiler_options_item(&tsconfig_json_content) {
                    insert_string.push_str(",");
                }
                block_string_new = regex2.replace(&block_string_new, insert_string).to_string();
            }

            result = regex.replace(&result, &block_string_new).to_string();
        } else {
            // compilerOptions 블록이 없는 경우
            panic!("tsconfig.json 에 \"compilerOptions\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");   
        }
    }
    result
}

fn apply_tsconfig_include_block(tsconfig_json_content: &str) -> String {
    let mut result: String = String::from(tsconfig_json_content);
    let pattern = r#""include":[^{}]*\[[^{}]*\]"#;
    let regex = Regex::new(&pattern).unwrap();
    if let Some(captured) = regex.captures(tsconfig_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(text) = captured.get(0) {
            // include 블록이 있는 경우
            let mut should_append_include_items: Vec<String> = Vec::new();

            let block_string: String = text.as_str().to_string();
            let mut block_string_new = String::from(block_string);
            let pattern2 = r#"\"include\":[^{}]*\["#;
            let regex2 = Regex::new(&pattern2).unwrap();

            if !block_string_new.contains(r#""./src/**/*""#) {
                should_append_include_items.push(r#""./src/**/*""#.to_string());
            }

            // torytis-build.tsx 에서 torytis-build.js 로 바뀌었으므로 제거하는 마이그레이션 코드로 변경하였음.
            if block_string_new.contains(r#""./torytis-build.tsx""#) {
                block_string_new = Regex::new(r#""./torytis-build.tsx"[^{},]*,"#).unwrap().replace(block_string_new.as_str(), "").to_string();
            }

            if !block_string_new.contains(r#""./torytis-env.d.ts""#) {
                should_append_include_items.push(r#""./torytis-env.d.ts""#.to_string());
            }

            if !block_string_new.contains(r#""./torytis-variable.d.ts""#) {
                should_append_include_items.push(r#""./torytis-variable.d.ts""#.to_string());
            }

            if !block_string_new.contains(r#""./torytis-variable-object.ts""#) {
                should_append_include_items.push(r#""./torytis-variable-object.ts""#.to_string());
            }

            if should_append_include_items.len() > 0 {
                let mut insert_string = format!("\"include\": [\n\t\t{}", should_append_include_items.join(",\n\t\t")); 
                if is_exist_tsconfig_include_item(&tsconfig_json_content) {
                    insert_string.push_str(",");
                } else {
                    insert_string.push_str("\n\t");
                }
                block_string_new = regex2.replace(&block_string_new, insert_string).to_string();
            }

            block_string_new = block_string_new.replace("\n\t\t\n", "\n");

            result = regex.replace(&result, &block_string_new).to_string();
        } else {
            // include 블록이 없는 경우
            panic!("tsconfig.json 에 \"include\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");   
        }
    } else {
        panic!("tsconfig.json 에 \"include\" 가 선언되어 있지 않습니다. 선언 후에 다시 시도해주세요.");   
    }
    result
}

fn apply_gitignore(content: &str) -> String {
    let mut result: String = String::from(content);
    if result.contains("torytis-build.js") {
        result = result.replace("\ntorytis-build.js", "").replace("torytis-build.js\n", "");
    }
    result
}

fn apply_tailwind_config_ts_content_block(content: &str) -> String {
    let mut result: String = String::from(content);

    let pattern = r#"content:[^\[\]]*\[[^\[\]]*\]"#;
    let regex = Regex::new(&pattern).unwrap();
    if let Some(captured) = regex.captures(&result) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(text) = captured.get(0) {
            let block_string: String = text.as_str().to_string();
            let mut block_string_new = String::from(block_string);

            // .torytis/index.css 제거
            block_string_new = Regex::new(r#"'.\/.torytis\/index.css'[^\,]*\,"#).unwrap().replace(&block_string_new, "").to_string();
            block_string_new = Regex::new(r#"\,[^\,]*'.\/.torytis\/index.css'"#).unwrap().replace(&block_string_new, "").to_string();

            result = regex.replace(result.as_str(), block_string_new).to_string();
        }
    }

    result
}

fn apply_torytis_build_js(content: &str) -> String {
    let mut result = String::new();
    println!("?");
    
    let pattern = r#"^(\s+)const indexJsx = await import\(convertIndexJsxPath\);"#;
    let const_index_jsx_line_regex = Regex::new(&pattern).unwrap();

    for line in content.lines() {
        if const_index_jsx_line_regex.is_match(line) {
            // 일치하는 부분을 공백으로 바꿉니다.
            let replaced_line = line.replace(r#"const indexJsx = await import(convertIndexJsxPath);"#, r#"const indexJsx = await import('./.torytis/index.js');"#);

            // 바뀐 라인을 결과 문자열에 추가합니다.
            result.push_str(&replaced_line);
            result.push('\n');
        } else {
            // 일치하지 않는 라인은 그대로 결과 문자열에 추가합니다.
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

fn is_exist_tsconfig_include_item(tsconfig_json_content: &str) -> bool {
    let mut result = false;
    let regex = Regex::new(r#""include"[^{}]*:[^{}]*\[[^{}]*"[^{}]*"[^{}]*\]"#).unwrap();

    if let Some(captured) = regex.captures(tsconfig_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(_) = captured.get(0) {
            result = true;
        }
    }
    result
}

fn is_exist_tsconfig_compiler_options_item(tsconfig_json_content: &str) -> bool {
    let mut result = false;
    let regex = Regex::new(r#""compilerOptions"[^{}]*:[^{}]*\{[^{}]*"[^{}]*"[^{}]*\}"#).unwrap();

    if let Some(captured) = regex.captures(tsconfig_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(_) = captured.get(0) {
            result = true;
        }
    }
    result
}

fn is_exist_package_json_scripts_item(package_json_content: &str) -> bool {
    let mut result = false;
    let regex = Regex::new(r#""scripts"[^{}]*:[^{}]*\{[^{}]*"[^{}]*"[^{}]*\}"#).unwrap();

    if let Some(captured) = regex.captures(package_json_content) {
        // 첫 번째 캡처된 그룹 출력
        if let Some(_) = captured.get(0) {
            result = true;
        }
    }
    result
}