use anyhow::{Ok, Result};
use gui::{displays::LargeDisplay, Gui};
use hms_common::app_dir_client::DefaultAppDirClient;
use hms_config::manager::HmsConfigManager;
use hms_db::manager::HmsDbManager;

mod gui;

#[derive(Debug)]
pub enum Mode {
    Large,
}

fn main() -> Result<()> {
    let app_dir_client = DefaultAppDirClient;
    let db_manager = HmsDbManager::new(&app_dir_client);
    let cfg_manager = HmsConfigManager::new(&app_dir_client);
    let cfg = cfg_manager.load_config()?;

    Gui::<LargeDisplay, DefaultAppDirClient>::run(&db_manager, cfg)?;
    Ok(())
}
