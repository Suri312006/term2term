use crate::{
    files::{Cache, Config, Paths},
    Handlers,
};

use color_eyre::Result;


#[derive(Debug)]
pub struct AppState {
    pub handlers: Handlers,
    pub cache: Cache,
    pub config: Config,
    pub paths: Paths,
}

impl AppState {
    pub async fn instantiate() -> Result<Self> {
        let paths = Paths::new()?;
        Ok(AppState {
            handlers: Handlers::new().await?,
            paths: Paths::new()?,
            config: Config::read(&paths)?,
            cache: Cache::read(&paths)?,
        })
    }
}
