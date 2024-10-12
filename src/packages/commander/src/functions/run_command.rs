use std::process::Stdio;

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