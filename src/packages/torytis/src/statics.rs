use include_dir::{include_dir, Dir};

pub static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");
pub static PROJECT_TEMPLATE_NAME: &str = "project-template";
pub static VERSION: &str = env!("CARGO_PKG_VERSION");