use std::path::PathBuf;


pub mod initialize;
pub mod config;

pub struct Paths {
    // do we really need these two?
    pub config_dir_path: PathBuf,
    pub config_file_path: PathBuf,
}
