use crate::cli::Command;
use anyhow::{Ok, Result};
use clap::Parser;
use cli::{Args, DisplayMode};
use gui::{
    displays::{LargeDisplay, SmallDisplay},
    Gui, GuiDisplay,
};
use hms_common::app_dir_client::{AppDirClient, DefaultAppDirClient};
use hms_config::manager::HmsConfigManager;
use hms_db::{manager::HmsDbManager, models::NewSnip};
use human_panic::setup_panic;
use std::{
    fs,
    io::{self, IsTerminal, Read},
};

mod cli;
mod gui;

fn main() {
    setup_panic!();
    if let Err(e) = prepare_environment() {
        eprintln!("Failed to prepare the application environment:\n{}", e);
        std::process::exit(1);
    }

    let args = Args::parse();

    match args.command {
        Some(Command::Add { snip, alias }) => {
            add_snip(snip, alias).unwrap_or_else(|e| {
                eprintln!("Failed to add snip:\n{}", e);
                std::process::exit(1);
            });
        }
        None => {
            match args.display_mode {
                DisplayMode::Small => run_gui::<SmallDisplay>(),
                DisplayMode::Large => run_gui::<LargeDisplay>(),
            }
            .unwrap_or_else(|e| {
                eprintln!("Failed to run GUI:\n{}", e);
                std::process::exit(1);
            });
        }
    }
}

fn prepare_environment() -> Result<()> {
    let app_dir_client = DefaultAppDirClient;
    let app_dir_path = app_dir_client.get_app_dir_path()?;

    if !app_dir_path.exists() {
        println!("Looks like this is your first time running Hold my Snip!");
        println!(
            "Creating application directory at {}",
            app_dir_path.display()
        );
        fs::create_dir_all(&app_dir_path)?;
    }

    initialize(&app_dir_client)?;

    Ok(())
}

fn add_snip(snip: Option<String>, alias: String) -> Result<()> {
    let snip_content = match snip {
        Some(content) => content,
        None => {
            if io::stdin().is_terminal() {
                eprintln!("Error: No snip provided, please provide a one.");
                std::process::exit(1);
            } else {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
                buffer.trim().to_string()
            }
        }
    };

    let new_snip = NewSnip::new(&alias, &snip_content);
    let app_dir_client = DefaultAppDirClient;
    let db_manager = HmsDbManager::new(&app_dir_client);
    db_manager.with_db(|db| db.insert_snip(&new_snip))?;
    Ok(())
}

fn initialize(app_dir_client: &DefaultAppDirClient) -> Result<()> {
    let db_manager = HmsDbManager::new(app_dir_client);
    let cfg_manager = HmsConfigManager::new(app_dir_client);

    if !cfg_manager.config_exists()? {
        let cfg = cfg_manager.wizard()?;
        println!("Storing configuration...");
        cfg_manager.save_config(&cfg)?;
    }

    if db_manager.db_has_pending_migrations()? {
        println!("Database migrations are pending. Running migrations...");
        db_manager.run_pending_migrations()?;
    }

    Ok(())
}

fn run_gui<D: GuiDisplay<DefaultAppDirClient>>() -> Result<()> {
    let app_dir_client = DefaultAppDirClient;
    let db_manager = HmsDbManager::new(&app_dir_client);
    let cfg_manager = HmsConfigManager::new(&app_dir_client);
    let cfg = cfg_manager.load_config()?;
    Gui::<D, DefaultAppDirClient>::run(&db_manager, cfg)
}
