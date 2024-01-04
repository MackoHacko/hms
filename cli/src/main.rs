#![allow(unused)]

use anyhow::{Ok, Result};
use app::HmsApp;
use gui::Gui;
use hms_common::app_dir_client::DefaultAppDirClient;
use hms_config::manager::HmsConfigManager;
use hms_db::{manager::HmsDbManager, models::NewSnip};

mod app;
mod commands;
mod gui;

fn main() -> Result<()> {
    let app_dir_client = DefaultAppDirClient;
    let cfg_manager = HmsConfigManager::new(&app_dir_client);
    let cfg = cfg_manager.load_config()?;
    let db_manager = HmsDbManager::new(&app_dir_client);
    let app = HmsApp::new(&app_dir_client);
    //app.run()

    Gui::run(cfg, db_manager)
}
