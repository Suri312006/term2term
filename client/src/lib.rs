use std::path::PathBuf;

pub mod api;
pub mod config;
pub mod initialize;
pub mod state;

pub struct Paths {
    // do we really need these two?
    pub t2t_dir_path: PathBuf,
    pub config_file_path: PathBuf,
    pub state_file_path: PathBuf,
}
