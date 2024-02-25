use crate::{
    cli::{Args, Command, DisplayMode, ImportCommand, StatsCommand},
    gui::{
        displays::{LargeDisplay, SmallDisplay},
        Gui,
    },
    import::csv::SnipCsv,
    stats::Stats,
};
use anyhow::{Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};
use hms_common::app_dir_client::{AppDirClient, DefaultAppDirClient};
use hms_db::{manager::HmsDbManager, models::NewSnip};
use human_panic::setup_panic;
use std::{
    fs,
    io::{self, IsTerminal, Read},
    path::PathBuf,
};

mod cli;
mod gui;
mod import;
mod stats;
mod term;

fn main() -> Result<()> {
    setup_panic!();
    let app_dir_client = DefaultAppDirClient;
    let db_manager = HmsDbManager::new(&app_dir_client);

    prepare_environment(&app_dir_client, &db_manager)?;

    let args = Args::parse();
    match args.command {
        Some(Command::Add { snip, alias }) => add_snip(&db_manager, snip, alias)?,
        Some(Command::Import(import_args)) => match import_args.command {
            ImportCommand::Csv { file } => insert_from_csv(&db_manager, file)?,
        },
        Some(Command::Stats(stats_args)) => match stats_args.command {
            StatsCommand::TopTen => Stats::access_count_top_list(&db_manager, 10)?,
        },
        None => run_gui(&db_manager, args.display_mode)?,
    }

    Ok(())
}

fn prepare_environment<A: AppDirClient>(
    app_dir_client: &A,
    db_manager: &HmsDbManager<A>,
) -> Result<()> {
    let app_dir_path = app_dir_client.get_app_dir_path()?;
    if !app_dir_path.exists() {
        println!("Looks like this is your first time running Hold my Snip! 🎉");
        let proceed: bool = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Set up required .hold_my_snip directory in home folder?")
            .interact()?;
        if !proceed {
            println!("That's too bad, exiting...");
            std::process::exit(0);
        }
        fs::create_dir_all(&app_dir_path)?;
        println!(
            "✅ Created application directory at {}",
            app_dir_path.display()
        );
    }
    if db_manager.db_has_pending_migrations()? {
        db_manager.run_pending_migrations()?;
        println!("✅ Ran pending db migrations");
    }

    Ok(())
}

fn add_snip<A: AppDirClient>(
    db_manager: &HmsDbManager<A>,
    snip: Option<String>,
    alias: String,
) -> Result<()> {
    let snip_content = snip
        .or_else(|| {
            if io::stdin().is_terminal() {
                eprintln!("Error: No snip provided, please provide a one.");
                std::process::exit(1);
            } else {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .expect("Failed to read from stdin");
                Some(buffer.trim().to_string())
            }
        })
        .unwrap();

    let new_snip = NewSnip::new(&alias, &snip_content);
    db_manager.with_db(|db| db.insert_snip(&new_snip))?;
    Ok(())
}

fn insert_from_csv(
    db_manager: &HmsDbManager<DefaultAppDirClient>,
    csv_file: PathBuf,
) -> Result<()> {
    let csv = SnipCsv::from_file(csv_file)?;
    let new_snips = csv.to_new_snips();
    db_manager.with_db(|db| db.insert_snips(&new_snips))?;
    Ok(())
}

fn run_gui<A: AppDirClient>(db_manager: &HmsDbManager<A>, display_mode: DisplayMode) -> Result<()> {
    match display_mode {
        DisplayMode::Small => Gui::<SmallDisplay, _>::run(db_manager),
        DisplayMode::Large => Gui::<LargeDisplay, _>::run(db_manager),
    }
}
