use hms_common::app_dir_client::AppDirClient;
use serde_derive::{Deserialize, Serialize};
use std::{fmt, fs};

mod constants;
pub mod error;
mod prelude;
mod wizard;

use crate::{constants::CONFIG_FILE, prelude::*};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HmsConfig {
    pub snip_limit: usize,
}

impl HmsConfig {
    pub fn default() -> Self {
        Self { snip_limit: 10 }
    }

    pub fn exists<P: AppDirClient>(app_dir_client: &P) -> Result<bool> {
        Ok(app_dir_client
            .get_app_dir_path()?
            .join(CONFIG_FILE)
            .exists())
    }

    pub fn load<P: AppDirClient>(app_dir_client: &P) -> Result<Self> {
        let config_path = app_dir_client.get_app_dir_path()?.join(CONFIG_FILE);
        let content = fs::read_to_string(config_path)?;
        toml::from_str(&content).map_err(Into::into)
    }

    pub fn save<P: AppDirClient>(&self, app_dir_client: &P) -> Result<()> {
        let content = toml::to_string(self)?;
        fs::write(
            app_dir_client.get_app_dir_path()?.join(CONFIG_FILE),
            content,
        )?;
        Ok(())
    }

    pub fn update_snip_limit<P: AppDirClient>(
        &self,
        app_dir_client: &P,
        snip_limit: usize,
    ) -> Result<()> {
        let mut updated = self.clone();
        updated.snip_limit = snip_limit;
        updated.save(app_dir_client)
    }
}

impl fmt::Display for HmsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Configuration:\n - Snip Limit: {}", self.snip_limit)
    }
}
