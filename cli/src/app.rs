use crate::{
    commands::{
        add::{add_cmd, ADD_CMD_ALIAS_ARG_ID, ADD_CMD_ID, ADD_CMD_SNIP_ARG_ID},
        init::{init_cmd, INIT_CMD_ID},
        main::main_cmd,
    },
    prelude::*,
};
use clap::ArgMatches;
use hms_common::app_dir_client::AppDirClient;
use hms_config::manager::HmsConfigManager;
use hms_db::{manager::HmsDbManager, models::NewSnip};
use std::fs;

pub struct HmsApp<'a, P>
where
    P: AppDirClient,
{
    app_dir_client: &'a P,
    cfg_manager: HmsConfigManager<'a, P>,
    db_manager: HmsDbManager<'a, P>,
}

impl<'a, P> HmsApp<'a, P>
where
    P: AppDirClient,
{
    pub fn new(app_dir_client: &'a P) -> Self {
        Self {
            app_dir_client,
            cfg_manager: HmsConfigManager::new(app_dir_client),
            db_manager: HmsDbManager::new(app_dir_client),
        }
    }

    pub fn run(&self) -> Result<()> {
        let cmd = main_cmd().subcommand(init_cmd()).subcommand(add_cmd());
        let mathches = cmd.get_matches();

        match mathches.subcommand() {
            Some((INIT_CMD_ID, _)) => self.handle_init_cmd(),
            Some((ADD_CMD_ID, args)) => self.handle_add_cmd(args),
            _ => unreachable!(),
        }
    }

    fn handle_init_cmd(&self) -> Result<()> {
        let app_dir = self.app_dir_client.get_app_dir_path()?;
        if !app_dir.exists() {
            fs::create_dir(app_dir)?;
        }
        let cfg = self.cfg_manager.wizard()?;
        self.cfg_manager.save_config(&cfg)?;
        if self.db_manager.db_has_pending_migrations()? {
            self.db_manager.run_pending_migrations()?;
        }
        Ok(())
    }

    fn needs_initialization(&self) -> Result<bool> {
        if !self.app_dir_client.get_app_dir_path()?.exists() {
            return Ok(true);
        }

        if !self.cfg_manager.config_exists()? {
            return Ok(true);
        }

        if self.db_manager.db_has_pending_migrations()? {
            return Ok(true);
        }

        Ok(false)
    }

    fn handle_add_cmd(&self, args: &ArgMatches) -> Result<()> {
        if self.needs_initialization()? {
            return Err(HmsError::NotInitialized);
        }

        let alias: &String = args
            .get_one(ADD_CMD_ALIAS_ARG_ID)
            .expect(&format!("`{}` is required", ADD_CMD_ALIAS_ARG_ID));

        let snip: &String = args
            .get_one(ADD_CMD_SNIP_ARG_ID)
            .expect(&format!("`{}` is required", ADD_CMD_SNIP_ARG_ID));

        let new_snip = NewSnip::new(alias, snip);
        self.db_manager.with_db(|db| db.insert_snip(&new_snip))?;
        println!("Snip added with alias {}", alias);
        Ok(())
    }
}
