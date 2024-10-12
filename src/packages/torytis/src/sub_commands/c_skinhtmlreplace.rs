use crate::functions::skin_html_replace::skin_html_replace;

#[derive(clap::Args)]
#[command(
    about=".torytis/skin.html 파일의 내용을 후처리 합니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    skin_html_replace(&true);
}