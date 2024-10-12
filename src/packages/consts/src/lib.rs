pub static TEMPLATE_NAME: &str = "torytis-start-template";
pub static TEMPLATE_VERSION: &str = "v0.0.5";

pub fn get_template_download_url() -> String {
  let url = format!("https://github.com/wisdomstar94/torytis/releases/download/{}-{}/{}.tar.gz", TEMPLATE_NAME, TEMPLATE_VERSION, TEMPLATE_NAME);
  url
}