use std::path::PathBuf;
use serde::Deserialize;

pub mod api;
pub mod config;
pub mod initialize;

pub struct Paths {
    // do we really need these two?
    pub config_dir_path: PathBuf,
    pub config_file_path: PathBuf,
}

#[derive(Deserialize)]
pub struct UserInfo {
    id: String,
    username: String,
}
