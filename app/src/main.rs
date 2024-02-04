use anyhow::Result;
use clap::Parser;
use cli::{Args, DisplayMode};
use gui::{
    displays::{LargeDisplay, SmallDisplay},
    Gui, GuiDisplay,
};
use hms_common::app_dir_client::DefaultAppDirClient;
use hms_config::manager::HmsConfigManager;
use hms_db::manager::HmsDbManager;

mod cli;
mod gui;

fn main() -> Result<()> {
    let args = Args::parse();
    match args.display_mode {
        DisplayMode::Small => run_gui::<SmallDisplay>(),
        DisplayMode::Large => run_gui::<LargeDisplay>(),
    }
}

fn run_gui<D: GuiDisplay<DefaultAppDirClient>>() -> Result<()> {
    let app_dir_client = DefaultAppDirClient;
    let db_manager = HmsDbManager::new(&app_dir_client);
    let cfg_manager = HmsConfigManager::new(&app_dir_client);
    let cfg = cfg_manager.load_config()?;
    Gui::<D, DefaultAppDirClient>::run(&db_manager, cfg)
}
