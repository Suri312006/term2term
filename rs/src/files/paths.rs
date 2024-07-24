use std::{path::PathBuf, str::FromStr};

use crate::{Error, Result};
use xdg_home::home_dir;

pub struct Paths {
    // do we really need these two?
    pub t2t_dir_path: PathBuf,
    pub config_file_path: PathBuf,
    pub state_file_path: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Paths> {
        let home_path = home_dir().ok_or(Error::from("Home Directory Not found"))?;
        let t2t_dir =
            PathBuf::from(String::from_str(home_path.to_str().unwrap())? + "/.config/term2term");
        let cfg_fp: PathBuf = [t2t_dir.to_str().unwrap(), "config.toml"].iter().collect();
        let state_fp: PathBuf = [t2t_dir.to_str().unwrap(), "state.toml"].iter().collect();
        Ok(Paths {
            t2t_dir_path: t2t_dir,
            config_file_path: cfg_fp,
            state_file_path: state_fp,
        })
    }
}
