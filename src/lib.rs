use std::process::Stdio;

use clap::Parser;
use regex::Regex;
pub mod sub_commands;
pub mod statics;
pub mod common;
pub mod structs;

#[derive(Parser)] // requires `derive` feature
#[command(name = "torytis")]
#[command(bin_name = "torytis")]
#[command(about = "torytis 는 tistory 블로그 스킨 개발 프레임워크 입니다.")]
enum Cli {
    // Subcommands...
    New(sub_commands::c_new::CliArgs), 
    Build(sub_commands::c_build::CliArgs), 
    Varbuild(sub_commands::c_varbuild::CliArgs), 
    // Migrate(sub_commands::c_migrate::CliArgs), 
    Dev(sub_commands::c_dev::CliArgs), 
    Version(sub_commands::c_version::CliArgs),
}

pub async fn run() {
    let parse_cli = Cli::parse();
    match parse_cli {
        Cli::New(args) => sub_commands::c_new::run(args),
        Cli::Build(args) => sub_commands::c_build::run(args),
        Cli::Varbuild(args) => sub_commands::c_varbuild::run(args),
        // Cli::Migrate(args) => sub_commands::c_migrate::run(args),
        Cli::Dev(args) => sub_commands::c_dev::run(args).await,
        Cli::Version(_) => sub_commands::c_version::run()
    }
}

pub fn run_command(cmd_string: &str) -> Result<std::process::Output, std::io::Error> {
    let output = if cfg!(target_os = "windows") {
      std::process::Command::new("cmd")
        .args(["/C", cmd_string])
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit())
        .output()
    } else {
      std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd_string)
        .stdout(Stdio::inherit()) 
        .stderr(Stdio::inherit())
        .output()
    };
    output
}

pub fn replace_skin_html_content(html_string: &String) -> String {
    let mut result = String::from(html_string);
    result = result.replacen("</head>", "<link href=\"./style.css\" as=\"style\" rel=\"preload\" /></head>", 1);
    result = result.replacen("</head>", "<link href=\"./style.css\" type=\"text/css\" rel=\"stylesheet\" /></head>", 1);
    result = result.replacen("</head>", "<script src=\"./images/script.js\"></script></head>", 1);
    result = result.replacen("<html", "<!DOCTYPE html><html", 1);
    result = result.replace("<tt_html_comment>", "<!-- ");
    result = result.replace("</tt_html_comment>", " -->");
    result = result.replace("<meta charSet", "<meta charset");
    result = result.replace("tt-onclick", "onclick");
    result = result.replace("tt-onmouseover", "onmouseover");
    result = result.replace("tt-onmouseout", "onmouseout");
    result = result.replace("tt-onmouseenter", "onmouseenter");
    result = result.replace("tt-onmouseleave", "onmouseleave");
    result = result.replace("tt-onkeypress", "onkeypress");
    result = result.replace("tt-onkeydown", "onkeydown");
    result = result.replace("tt-value", "value");
    result = result.replace("tt-onload", "onload");
    result = result.replace("tt-onerror", "onerror");

    let pattern = r#"tt-onlyattr=\"(.*?)\""#;
    let re = Regex::new(pattern).unwrap();
    let output_string = re.replace_all(&result.as_str(), r#"$1"#);
    result = output_string.to_string();
    result
}