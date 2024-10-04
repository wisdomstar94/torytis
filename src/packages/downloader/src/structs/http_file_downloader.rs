use std::path::{Path, PathBuf};
use tokio::{
    fs,                 // fs::File
    io::AsyncWriteExt , // write_all
};

use super::{custom_error::CustomError, file_name_options::FileNameOptions};

#[derive(Debug)]
pub struct HttpFileDownloader {
    client: reqwest::Client,
}

impl Default for HttpFileDownloader {
    fn default() -> Self {
        Self {
            client: Default::default(),
        }
    }
}

impl HttpFileDownloader {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
        }
    }

    // destination : 다운로드 받을 폴더 위치
    // url : 다운로드할 파일 url 문자열
    pub async fn download(&self, destination_folder: impl AsRef<Path>, url: &str, file_name_options: FileNameOptions) -> Result<PathBuf, CustomError> {     
        // 파일 저장할 폴더 path 얻기
        let destination = destination_folder.as_ref();
        
        // 파일 저장할 폴더 생성
        if !destination.exists() {
            fs::create_dir_all(destination).await?
        }

        // https 파일 요청
        let mut response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?;
        
        // for (name, value) in response.headers() {
        //     println!("name: {:?}, value: {:?}", name, value);
        // }

        // response header 로부터 파일 이름 가져오기
        let content_disposition_option = response.headers().get("content-disposition");
        let mut remote_filename_option: Option<String> = None;
        if let Some(content_disposition) = content_disposition_option {
            let content_disposition_string = content_disposition.to_str().unwrap();
            // println!("content_disposition_string is {}", content_disposition_string);
            // let content_disposition_string_split: Vec<&str> = text.split(">>").collect();
            // attachment; filename=
            let mut result = String::from(content_disposition_string);
            result = result.replacen("attachment; filename=", "", 1);
            let filename = result.trim();
            remote_filename_option = Some(filename.to_string());
        }

        // filename 설정
        let finally_filename_option: Option<String> = match file_name_options {
            FileNameOptions::UseCustomFileName(filename) => Some(filename),
            FileNameOptions::UseRemoteFileName(default_filename) => {
                if let Some(remote_filename) = remote_filename_option {
                    Some(remote_filename)
                } else {
                    Some(default_filename)
                }
            }
            // Cli::New(args) => sub_commands::c_new::run(args).await,
            // Cli::Build(args) => sub_commands::c_build::run(args),
            // Cli::Varbuild(args) => sub_commands::c_varbuild::run(args),
            // // Cli::Migrate(args) => sub_commands::c_migrate::run(args),
            // Cli::Dev(args) => sub_commands::c_dev::run(args).await,
            // Cli::Version(_) => sub_commands::c_version::run()
        };

        if let None = finally_filename_option {
            panic!("filename not initialized!");
        }

        let finally_filename = finally_filename_option.unwrap();

        // path를 파일명을 포함한 full file path로 변경
        let path = destination.join(finally_filename);

        // 파일 생성 및 열기
        let mut file = fs::File::create(&path).await?;

        // 청크 단위로 파일다운로드 및 파일에 저장
        while let Some(chunk) = response.chunk().await? {
            file.write_all(&chunk).await?;
        }

        Ok(path)
    }
}