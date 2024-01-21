use std::{env, path::Path, fs};
use crate::{run_command, statics::{PROJECT_TEMPLATE_NAME, STATIC_DIR}};

#[derive(clap::Args)]
#[command(
  about="새로운 torytis 프로젝트를 생성합니다.", 
  long_about = None)
]
pub struct CliArgs {
    /// 프로젝트명을 입력하세요.
    #[arg(short='n', long="name")]
    name: Option<String>,
}

pub fn run(args: CliArgs) {
    if let None = args.name {
        panic!("--name 인자를 입력해주세요.");
    }

    let project_name = args.name.unwrap();

    let working_dir_path_buf = env::current_dir().unwrap();
    let working_dir = working_dir_path_buf.to_str().unwrap();
    let project_dir_path_buf = Path::new(working_dir).join(project_name.as_str());

    // step 0) project dir 존재 유무 체크
    if let Ok(_) = fs::metadata(project_dir_path_buf.as_path()) {
        let msg = format!("{} 폴더가 이미 존재하여 프로젝트를 생성할 수 없습니다.", project_name);
        panic!("{}", msg.as_str());
    }

    // step 1) project dir 생성
    fs::create_dir_all(project_dir_path_buf.as_path()).unwrap();

    // step 2) 파일 생성
    let glob = format!("{}/**/*", PROJECT_TEMPLATE_NAME);
    for entry in STATIC_DIR.find(&glob).unwrap() {
        if let Some(file) = entry.as_file(){
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            let convert_path_str = path_str.replace(PROJECT_TEMPLATE_NAME, &project_name);
            let convert_path = Path::new(convert_path_str.as_str());
            let file_name = path.file_name().unwrap();
            println!("create file : {:?}", convert_path);
            // println!("file_name : {:?}", file_name);
            // let file = STATIC_DIR.get_file(path).unwrap();
            let file_content_original = file.contents_utf8().unwrap();
            let mut file_content_convert: Option<String> = None;
            if file_name == "package.json" {
                file_content_convert = Some(file_content_original.replace("project__name", &project_name));
            }

            // println!("file_content is {:?}", file_content);
            if let Some(content) = file_content_convert {
                fs::write(convert_path, content).unwrap();
            } else {
                fs::write(convert_path, String::from(file_content_original)).unwrap();
            }
        } else if let Some(dir) = entry.as_dir()  {
            let path = dir.path();
            let path_str = path.to_str().unwrap();
            let convert_path_str = path_str.replace(PROJECT_TEMPLATE_NAME, &project_name);
            let convert_path = Path::new(convert_path_str.as_str());
            fs::create_dir_all(convert_path).unwrap();
        }
    }

    // step 3) npm install 진행
    let project_dir_path_str = project_dir_path_buf.to_str().unwrap();
    println!("created project dir : {:#?}", project_dir_path_str);

    if env::consts::OS == "windows" {
        let bat_file_path_buf = project_dir_path_buf.join("torytis-temp.bat");
        let bat_file_path = bat_file_path_buf.as_path();
        let mut bat_file_content = String::new();
        bat_file_content.push_str(format!("pushd {}\n", project_dir_path_str).as_str());
        bat_file_content.push_str("npm i\n");
        bat_file_content.push_str("popd");

        let bat_file_content_euc_kr = encoding_rs::EUC_KR.encode(bat_file_content.as_str());
        fs::write(bat_file_path, bat_file_content_euc_kr.0).unwrap();
        println!("npm installing...");
        run_command(bat_file_path.to_str().unwrap()).unwrap();
        fs::remove_file(bat_file_path).unwrap();
    } else {
        println!("npm installing...");
        let command = format!("npm install --prefix {}", project_dir_path_str);
        run_command(command.as_str()).unwrap();
    }
}