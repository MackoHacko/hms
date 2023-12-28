use crate::{prelude::*, HmsConfig};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

impl HmsConfig {
    pub fn wizard() -> Result<Self> {
        match Self::ask_default()? {
            true => Ok(Self::default()),
            false => {
                let snip_limit = Self::ask_snip_limit()?;
                Ok(Self { snip_limit })
            }
        }
    }

    fn ask_default() -> Result<bool> {
        Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Use default config? {}", Self::default()))
            .interact()
            .map_err(Into::into)
    }

    fn ask_snip_limit() -> Result<usize> {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Snip limit?")
            .default(10)
            .interact()
            .map_err(Into::into)
    }
}
