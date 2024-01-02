use crate::{constants, models, prelude};
use constants::CONFIG_FILE;
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use hms_common::app_dir_client::AppDirClient;
use models::HmsConfig;
use prelude::*;
use std::fs;

pub struct HmsConfigManager<'a, P>
where
    P: AppDirClient,
{
    pub app_dir_client: &'a P,
}

impl<'a, P> HmsConfigManager<'a, P>
where
    P: AppDirClient,
{
    pub fn new(app_dir_client: &'a P) -> HmsConfigManager<P> {
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

    pub fn update_snip_limit(&self, snip_limit: i64) -> Result<()> {
        let mut updated = self.load_config()?;
        updated.snip_limit = snip_limit;
        self.save_config(&updated)
    }

    pub fn wizard(&self) -> Result<HmsConfig> {
        if self.config_exists()? {
            let existing_cfg = self.load_config()?;
            if Self::ask_use_existing(&existing_cfg)? {
                return Ok(existing_cfg);
            }
        }
        match Self::ask_default()? {
            true => Ok(HmsConfig::default()),
            false => {
                let snip_limit = Self::ask_snip_limit()?;
                Ok(HmsConfig { snip_limit })
            }
        }
    }

    fn ask_use_existing(cfg: &HmsConfig) -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Use existing config?\n{}", cfg))
            .interact()
            .map_err(Into::into)
    }

    fn ask_default() -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Use default config?\n{}", HmsConfig::default()))
            .interact()
            .map_err(Into::into)
    }

    fn ask_snip_limit() -> Result<i64> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Snip limit?")
            .default(HmsConfig::default().snip_limit)
            .interact()
            .map_err(Into::into)
    }
}
