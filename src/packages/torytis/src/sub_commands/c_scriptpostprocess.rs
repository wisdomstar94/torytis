use crate::functions::script_postprocess::script_postprocess;

#[derive(clap::Args)]
#[command(
    about=".torytis/script.js 파일의 내용을 후처리 합니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    script_postprocess(&false);
}