// use std::{
//     fs::{create_dir, File, OpenOptions},
//     io::{ErrorKind, Read, Write},
// };
//
// use serde::{Deserialize, Serialize};
//
// use crate::Result;
//
//
// #[derive(Deserialize, Serialize, Debug)]
// pub struct Config {
//     pub theme: Theme,
//     pub users: Vec<User>,
// }
//
// #[derive(Deserialize, Serialize, Debug)]
// pub enum Theme {
//     Default,
// }
//
// impl Config {
//     pub fn read() -> Result<Config> {
//         let paths = Paths::new()?;
//         let mut cfg_file = File::open(paths.config_file_path)?;
//         let mut buf = String::new();
//         cfg_file.read_to_string(&mut buf)?;
//
//         let config: Config = toml::from_str(buf.as_str())
//             .with_context(|| "Unsupported structure for config file.")?;
//
//         Ok(config)
//     }
//
//     pub fn check_existing() -> Result<bool> {
//         let paths = Paths::new()?;
//         match File::open(paths.config_file_path.clone()) {
//             Ok(_) => Ok(true),
//             Err(err) => {
//                 if err.kind() == ErrorKind::NotFound {
//                     return Ok(false);
//                 }
//
//                 Err(anyhow!(err.to_string()))
//             }
//         }
//     }
//
//     //TODO: i would really like to use toml_edit to do this so comments and stuff
//     //are preserved but it is what it is you know
//     pub fn write(&self) -> Result<()> {
//         let paths = Paths::new()?;
//
//         match create_dir(&paths.t2t_dir_path) {
//             Ok(()) => {}
//             Err(err) => {
//                 if err.kind() != ErrorKind::AlreadyExists {
//                     return Err(anyhow!(
//                         "Weird error while creating directory. {}",
//                         err.to_string()
//                     ));
//                 }
//             }
//         };
//
//         let mut cfg_file = match OpenOptions::new()
//             .truncate(true)
//             .write(true)
//             .read(true)
//             .open(paths.config_file_path.clone())
//         {
//             Ok(file) => Ok(file),
//
//             Err(err) => match err.kind() {
//                 ErrorKind::NotFound => Ok(File::create(paths.config_file_path.clone()).unwrap()),
//                 _ => Err(anyhow!(
//                     "there was an error trying to write defualt config file {}",
//                     err.to_string()
//                 )),
//             },
//         }?;
//
//         let file_data = toml::to_string(&self)?;
//         cfg_file.write_all(file_data.as_bytes())?;
//         Ok(())
//     }
//
//     pub fn default() -> Config {
//         Config {
//             theme: Theme::Default,
//             users: vec![],
//         }
//     }
// }
