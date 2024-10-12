use crate::functions::move_public_to_dot_torytis::move_public_to_dot_torytis;

#[derive(clap::Args)]
#[command(
    about="src/public 밑의 파일들을 .torytis/images 밑으로 복사합니다.", 
    long_about = None
)]
pub struct CliArgs {
    // #[arg(short='n', long="name")]
    // name: Option<String>,
}

pub fn run(_: CliArgs) {
    move_public_to_dot_torytis(&false);
}