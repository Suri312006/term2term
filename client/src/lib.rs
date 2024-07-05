use std::path::PathBuf;


pub mod config;
pub mod initialize;



pub struct Paths {
    // do we really need these two?
    pub config_dir_path: PathBuf,
    pub config_file_path: PathBuf,
}
