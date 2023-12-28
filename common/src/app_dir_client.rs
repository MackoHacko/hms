use std::{
    io::{Error, ErrorKind},
    path::PathBuf,
};

const APP_DIR: &str = ".hold_my_snip";

pub trait AppDirClient {
    fn get_app_dir_path(&self) -> Result<PathBuf, Error>;
}

pub struct DefaultAppDirClient;

impl AppDirClient for DefaultAppDirClient {
    fn get_app_dir_path(&self) -> Result<PathBuf, Error> {
        dirs::home_dir()
            .map(|hd| hd.join(APP_DIR))
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Unable to locate home directory"))
    }
}
