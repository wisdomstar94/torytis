use std::{env, fs::{self, DirEntry}, io, collections::HashMap};
use glob::glob;
use serde_json::Value;
use crate::{run_command, replace_skin_html_content};

#[derive(clap::Args)]
#[command(
  about="torytis 프로젝트를 빌드합니다.", 
  long_about = None)
]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    let working_dir_path_buf = env::current_dir().unwrap();
    let torytis_build_js_file_path_buf = working_dir_path_buf.join("torytis-build.js");
    let dot_torytis_index_xml_path_buf = working_dir_path_buf.join(".torytis").join("index.xml");
    let dot_torytis_index_xml_path = dot_torytis_index_xml_path_buf.as_path();
    let src_public_index_xml_path_buf = working_dir_path_buf.join("src").join("public").join("index.xml");
    let src_public_index_xml_path = src_public_index_xml_path_buf.as_path();

    // src/public/index.xml 체크
    if let Err(_) = fs::metadata(src_public_index_xml_path) {
        panic!("src/public/index.xml 파일이 존재하지 않습니다. 해당 파일을 생성 후 다시 시도해주세요. (https://tistory.github.io/document-tistory-skin/common/index.xml.html)")
    }

    // .torytis 폴더 체크 및 생성
    let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
    let dot_torytis_dir_path = dot_torytis_dir_path_buf.as_path();
    if let Err(_) = fs::metadata(dot_torytis_dir_path) {
        fs::create_dir_all(dot_torytis_dir_path).unwrap();
    }
    
    // src/**/*.script.tsx 파일들을 읽어서 .torytis/script.ts 파일 만들기
    let dot_torytis_script_ts_path_buf = dot_torytis_dir_path_buf.join("script.ts");
    let mut script_ts_content = String::from("");
    let target_glob_path_buf = working_dir_path_buf.join("src").join("**").join("*.script.tsx");
    let target_glob_str = target_glob_path_buf.as_path().to_str().unwrap();
    for entry in glob(target_glob_str).expect("Failed to read glob pattern") {
        if let Ok(path_buf) = entry {
            let absolute_path_str = path_buf.to_str().unwrap();
            let relative_path_str = absolute_path_str.replace(working_dir_path_buf.to_str().unwrap(), "");          
            // println!("relative_path_str : {}", relative_path_str);
            script_ts_content.push_str(format!("import \"..{}\";\n", relative_path_str).replace("\\", "/").as_str());
        }
    }
    fs::write(dot_torytis_script_ts_path_buf.as_path(), script_ts_content).unwrap();

    // js 를 필요로 하는 로직 실행
    {
        let command = format!("node {}", torytis_build_js_file_path_buf.to_str().unwrap());
        println!("> {}", command);
        let _ = run_command(command.as_str()).unwrap();
        // println!("<- {:?}", output);
    }

    // 현재 시점 .torytis 에 존재하는 파일들
    // index.css
    // script.js
    // script.ts
    // skin.html

    // script.ts 파일 삭제
    fs::remove_file(dot_torytis_script_ts_path_buf.as_path()).unwrap();

    // tailwind 빌드하기
    // let tailwind_config_ts_file_path_buf = working_dir_path_buf.join("tailwind.config.ts");
    // {
    //     let command = format!("npm run tsc -- --esModuleInterop {}", tailwind_config_ts_file_path_buf.to_str().unwrap());
    //     println!("> {}", command);
    //     let _ = run_command(command.as_str()).unwrap();
    // }
    let torytis_dot_index_css_file_path_buf = dot_torytis_dir_path_buf.join("index.css");
    let torytis_dot_style_css_file_path_buf = dot_torytis_dir_path_buf.join("style.css");
    let tailwind_build_command = format!("npm run tailwindcss -- -c ./tailwind.config.ts -i {} -o {}", torytis_dot_index_css_file_path_buf.to_str().unwrap(), torytis_dot_style_css_file_path_buf.to_str().unwrap());
    println!("> {}", tailwind_build_command);
    {
        let _ = run_command(tailwind_build_command.as_str()).unwrap();
        // println!("<- {:?}", output);
    }

    // index.css 파일 삭제
    fs::remove_file(torytis_dot_index_css_file_path_buf.as_path()).unwrap();

    // skin.html 파일 내용 치환하기
    let dot_torytis_skin_html_file_path_buf = dot_torytis_dir_path_buf.join("skin.html");
    let dot_torytis_skin_html_file_path = dot_torytis_skin_html_file_path_buf.as_path();
    let skin_html_string = fs::read_to_string(dot_torytis_skin_html_file_path).unwrap();
    let skin_html_string_convert = replace_skin_html_content(&skin_html_string);
    fs::write(dot_torytis_skin_html_file_path, skin_html_string_convert).unwrap();

    // src/public 폴더 밑에 있는 파일들을 모두 .torytis/ 폴더 밑으로 복사하기
    let src_public_dir_path_buf = working_dir_path_buf.join("src").join("public");
    let read_dir_result = fs::read_dir(src_public_dir_path_buf.as_path()).unwrap();
    let read_dir_result_vec: Vec<io::Result<DirEntry>> = read_dir_result.collect();
    for item in read_dir_result_vec {
        let entry = item.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            continue;
        }
        let entry_path_buf = entry.path();
        let after_path_bug = dot_torytis_dir_path_buf.join(entry.file_name());
        fs::copy(entry_path_buf.as_path(), after_path_bug).unwrap();
    }

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