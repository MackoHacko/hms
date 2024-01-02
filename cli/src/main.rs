use app::HmsApp;
use hms_common::app_dir_client::DefaultAppDirClient;
use prelude::*;

mod app;
mod commands;
mod errors;
mod prelude;

fn main() -> Result<()> {
    let app = HmsApp::new(&DefaultAppDirClient);
    app.run().map_err(|err| match err {
        HmsError::IO(e) => {
            eprintln!("IO error occurred: {}", e);
            std::process::exit(1);
        }
        HmsError::ConfigError(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
        HmsError::DbError(e) => {
            eprintln!("Database error: {}", e);
            std::process::exit(1);
        }
        HmsError::NotInitialized => {
            eprintln!("Application is not initialized. Please run the init command first.");
            std::process::exit(1);
        }
    })
}
