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
    /// true : images/ 폴더 없는 구조로 빌드, false : images/ 폴더 구조로 빌드 (기본 값: true)
    #[arg(short='f', long="flat")]
    flat: Option<bool>,
}

pub fn run(args: CliArgs) {
    let flat = args.flat.unwrap_or_else(|| true);

    let root_filenames: Vec<&str> = vec!["index.xml", "preview1600.jpg", "preview256.jpg", "preview560.jpg", "skin.html", "style.css"];

    let working_dir_path_buf = env::current_dir().unwrap();
    let torytis_build_js_file_path_buf = working_dir_path_buf.join("torytis.build.mjs");
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
    } else {
        fs::remove_dir_all(dot_torytis_dir_path).unwrap();
        fs::create_dir_all(dot_torytis_dir_path).unwrap();
    }

    // flat 이 false 일 경우
    let dot_torytis_images_dir_path_buf = dot_torytis_dir_path_buf.join("images/");
    let dot_torytis_images_dir_path = dot_torytis_images_dir_path_buf.as_path();
    if !flat {
        if let Err(_) = fs::metadata(dot_torytis_images_dir_path) {
            fs::create_dir_all(dot_torytis_images_dir_path).unwrap();
        } 
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
    // style.css
    // script.js
    // script.ts
    // skin.html

    // script.ts 파일 삭제
    fs::remove_file(dot_torytis_script_ts_path_buf.as_path()).unwrap();

    let script_js_file_path_buf = dot_torytis_dir_path_buf.join("script.js");
    let images_script_js_file_path_buf = dot_torytis_dir_path_buf.join("images").join("script.js");
    if !flat {
        fs::rename(script_js_file_path_buf.as_path(), images_script_js_file_path_buf.as_path()).unwrap()
    }

    // script.js 파일 내용 수정
    let script_js_file_content = fs::read_to_string(script_js_file_path_buf.clone()).unwrap();
    let mut new_script_js_file_content = String::from("");
    new_script_js_file_content.push_str("(function(){\n");
    new_script_js_file_content.push_str(script_js_file_content.as_str());
    new_script_js_file_content.push_str("\n})();");
    fs::write(script_js_file_path_buf.clone(), new_script_js_file_content).unwrap();

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
        let filename = entry_path_buf.as_path().file_name().unwrap().to_str().unwrap();

        let after_path_buf = if !flat && !root_filenames.contains(&filename) {
            dot_torytis_dir_path_buf.join("images").join(filename)
        } else {
            dot_torytis_dir_path_buf.join(filename)
        };

        fs::copy(entry_path_buf.as_path(), after_path_buf).unwrap();
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

    // // .torytis/ 밑에 있는 파일들을 티스토리에 업로드 할 경우 변경되는 실제 구조에 맞춰 .torytis-real-struct 로 복사하기
    // let dot_torytis_real_struct_dir_path_buf = working_dir_path_buf.join(".torytis-real-struct");
    // let dot_torytis_real_struct_dir_path = dot_torytis_real_struct_dir_path_buf.as_path();
    // if let Err(_) = fs::metadata(dot_torytis_real_struct_dir_path) {
    //     fs::create_dir_all(dot_torytis_real_struct_dir_path).unwrap();
    // }
    // let dot_torytis_real_struct_images_dir_path_buf = dot_torytis_real_struct_dir_path_buf.join("images");
    // let dot_torytis_real_struct_images_dir_path = dot_torytis_real_struct_images_dir_path_buf.as_path();
    // if let Err(_) = fs::metadata(dot_torytis_real_struct_images_dir_path) {
    //     fs::create_dir_all(dot_torytis_real_struct_images_dir_path).unwrap();
    // }
    // let dot_torytis_under_all_files_path_buf = dot_torytis_dir_path_buf.join("**").join("*");
    // let target_glob_str = dot_torytis_under_all_files_path_buf.as_path().to_str().unwrap();
    // for entry in glob(target_glob_str).expect("Failed to read glob pattern") {
    //     if let Ok(path_buf) = entry {
    //         let absolute_path = path_buf.as_path();
    //         let filename = absolute_path.file_name().unwrap().to_str().unwrap();
    //         let root_filenames: Vec<&str> = vec!["index.xml", "preview1600.jpg", "preview256.jpg", "preview560.jpg", "skin.html", "style.css"];
    //         let copy_path_buf = if root_filenames.contains(&filename) {
    //             dot_torytis_real_struct_dir_path_buf.join(filename)
    //         } else {
    //             dot_torytis_real_struct_dir_path_buf.join("images").join(filename)
    //         };
    //         fs::copy(absolute_path, copy_path_buf.as_path()).unwrap();
    //     }
    // }
}