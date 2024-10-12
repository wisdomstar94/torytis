use crate::functions::build_preprocess::build_preprocess;

#[derive(clap::Args)]
#[command(
    about="빌드를 하기 전에 먼저 진행해야 하는 작업을 진행합니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    build_preprocess(&false);
}