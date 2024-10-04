use std::{env, path::Path, fs};
use consts::{get_template_download_url, TEMPLATE_NAME, TEMPLATE_VERSION};
use downloader::structs::{file_name_options::FileNameOptions, http_file_downloader::HttpFileDownloader};
use file_manager::structs::file_content_controller::FileContentController;
use flater::functions::unpack::unpack_tar_gz;

use crate::run_command;

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

pub async fn run(args: CliArgs) {
    if let None = args.name {
        panic!("--name 인자를 입력해주세요.");
    }

    let project_name = args.name.unwrap();

    let working_dir_path_buf = env::current_dir().unwrap();
    let working_dir = working_dir_path_buf.to_str().unwrap();
    let project_dir_path_buf = Path::new(working_dir).join(project_name.as_str());

    // step 1) project dir 존재 유무 체크
    if let Ok(_) = fs::metadata(project_dir_path_buf.as_path()) {
        let msg = format!("{} 폴더가 이미 존재하여 프로젝트를 생성할 수 없습니다.", project_name);
        panic!("{}", msg.as_str());
    }

    // step 2) project dir 생성
    fs::create_dir_all(project_dir_path_buf.as_path()).unwrap();

    // step 3) template 다운로드
    println!("downloading start template [{} {}]", TEMPLATE_NAME, TEMPLATE_VERSION);

    let template_download_url = get_template_download_url();
    let template_tar_gz_file_path = HttpFileDownloader::default().download(
        project_dir_path_buf.as_path(), 
        template_download_url.as_str(),
        FileNameOptions::UseRemoteFileName(String::from("no_named"))
    ).await.unwrap();

    println!("downloaded! start template [{} {}]", TEMPLATE_NAME, TEMPLATE_VERSION);

    // step 4) 압축 해제
    unpack_tar_gz(&template_tar_gz_file_path, &project_dir_path_buf).unwrap();

    // step 5) 압축 파일 삭제
    fs::remove_file(&template_tar_gz_file_path).unwrap();

    // step 6) README.md, CHANGELOG.md 파일 삭제
    let readme_file_path = project_dir_path_buf.join("README.md");
    let changelog_file_path = project_dir_path_buf.join("CHANGELOG.md");
    fs::remove_file(&readme_file_path).unwrap();
    fs::remove_file(&changelog_file_path).unwrap();

    // step 7) package.json 파일 내용 수정
    let package_json_file_path = project_dir_path_buf.join("package.json");
    FileContentController::new(package_json_file_path)
        .change(|file_content| {
            file_content.replacen("@wisdomstar94/torytis-start-template", &project_name, 1)
        })
        .commit();

    // step 8) npm install 진행
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