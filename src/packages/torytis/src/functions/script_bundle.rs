use std::{env, fs, path::PathBuf};
use glob::glob;

pub fn script_bundle() -> PathBuf {
  // src/**/*.script.tsx 파일들을 읽어서 .torytis/script.ts 파일 만들기
  let working_dir_path_buf = env::current_dir().unwrap();
  let dot_torytis_dir_path_buf = working_dir_path_buf.join(".torytis/");
  let dot_torytis_script_ts_path_buf = dot_torytis_dir_path_buf.join("script.ts");
  let mut script_ts_content = String::from("");
  let target_glob_path_buf = working_dir_path_buf.join("src").join("**").join("*.script.tsx");
  let target_glob_str = target_glob_path_buf.as_path().to_str().unwrap();
  for entry in glob(target_glob_str).expect("Failed to read glob pattern") {
      if let Ok(path_buf) = entry {
          let absolute_path_str = path_buf.to_str().unwrap();
          let relative_path_str = absolute_path_str.replace(working_dir_path_buf.to_str().unwrap(), "");          
          // println!("relative_path_str : {}", relative_path_str);
          script_ts_content.push_str(format!("import \"..{}\";\n", relative_path_str).replace("\\", "/").as_str());
      }
  }
  fs::write(dot_torytis_script_ts_path_buf.as_path(), script_ts_content).unwrap();
  return dot_torytis_script_ts_path_buf;
}