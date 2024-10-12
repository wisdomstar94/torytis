use clap::Parser;
pub mod sub_commands;
pub mod statics;
pub mod common;
pub mod structs;
pub mod functions;

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
    Scriptbundle(sub_commands::c_scriptbundle::CliArgs),
    Scriptpostprocess(sub_commands::c_scriptpostprocess::CliArgs),
    Skinhtmlreplace(sub_commands::c_skinhtmlreplace::CliArgs),
    Buildpreprocess(sub_commands::c_buildpreprocess::CliArgs),
    Movepublictodottorytis(sub_commands::c_movepublictodottorytis::CliArgs),
}

pub async fn run() {
    let parse_cli = Cli::parse();
    match parse_cli {
        Cli::New(args) => sub_commands::c_new::run(args).await,
        Cli::Build(args) => sub_commands::c_build::run(args),
        Cli::Varbuild(args) => sub_commands::c_varbuild::run(args),
        // Cli::Migrate(args) => sub_commands::c_migrate::run(args),
        Cli::Dev(args) => sub_commands::c_dev::run(args).await,
        Cli::Version(_) => sub_commands::c_version::run(),
        Cli::Scriptbundle(args) => sub_commands::c_scriptbundle::run(args),
        Cli::Scriptpostprocess(args) => sub_commands::c_scriptpostprocess::run(args),
        Cli::Skinhtmlreplace(args) => sub_commands::c_skinhtmlreplace::run(args),
        Cli::Buildpreprocess(args) => sub_commands::c_buildpreprocess::run(args),
        Cli::Movepublictodottorytis(args) => sub_commands::c_movepublictodottorytis::run(args),
    }
}

