use crate::{constants, models, prelude};
use constants::CONFIG_FILE;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use hms_common::app_dir_client::AppDirClient;
use models::HmsConfig;
use prelude::*;
use std::fs;

pub struct HmsConfigManager<P>
where
    P: AppDirClient,
{
    pub app_dir_client: P,
}

impl<P> HmsConfigManager<P>
where
    P: AppDirClient,
{
    pub fn new(app_dir_client: P) -> HmsConfigManager<P> {
        HmsConfigManager { app_dir_client }
    }

    pub fn config_exists(&self) -> Result<bool> {
        Ok(self
            .app_dir_client
            .get_app_dir_path()?
            .join(CONFIG_FILE)
            .exists())
    }

    pub fn load_config(&self) -> Result<HmsConfig> {
        let config_path = self.app_dir_client.get_app_dir_path()?.join(CONFIG_FILE);
        let content = fs::read_to_string(config_path)?;
        toml::from_str(&content).map_err(Into::into)
    }

    pub fn save_config(&self, cfg: &HmsConfig) -> Result<()> {
        let content = toml::to_string(cfg)?;
        fs::write(
            self.app_dir_client.get_app_dir_path()?.join(CONFIG_FILE),
            content,
        )?;
        Ok(())
    }

    pub fn update_snip_limit(&self, snip_limit: usize) -> Result<()> {
        let mut updated = self.load_config()?;
        updated.snip_limit = snip_limit;
        self.save_config(&updated)
    }

    pub fn wizard() -> Result<HmsConfig> {
        match Self::ask_default()? {
            true => Ok(HmsConfig::default()),
            false => {
                let snip_limit = Self::ask_snip_limit()?;
                Ok(HmsConfig { snip_limit })
            }
        }
    }

    fn ask_default() -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Use default config? {}", HmsConfig::default()))
            .interact()
            .map_err(Into::into)
    }

    fn ask_snip_limit() -> Result<usize> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Snip limit?")
            .default(HmsConfig::default().snip_limit)
            .interact()
            .map_err(Into::into)
    }
}
