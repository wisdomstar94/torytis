use std::env;

#[derive(clap::Args)]
#[command(
  about="torytis 버전을 출력합니다.", 
  long_about = None)
]
pub struct CliArgs {
    
}

pub fn run(
    //args: CliArgs
) {
    let version = env!("CARGO_PKG_VERSION");
    println!("v{}", version);
}