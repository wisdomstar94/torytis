use crate::functions::script_bundle::script_bundle;

#[derive(clap::Args)]
#[command(
    about="src/**/*.script.tsx 파일들을 .torytis/script.ts 파일 하나로 묶습니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    script_bundle();
}