use crate::{constants::DB_FILE, db::HmsDb, prelude::*};
use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use hms_common::app_dir_client::AppDirClient;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct HmsDbManager<P>
where
    P: AppDirClient,
{
    pub app_dir_client: P,
}

impl<P> HmsDbManager<P>
where
    P: AppDirClient,
{
    pub fn new(app_dir_client: P) -> HmsDbManager<P> {
        HmsDbManager { app_dir_client }
    }

    pub fn db_exists(&self) -> Result<bool> {
        let db_path = self.app_dir_client.get_app_dir_path()?.join(DB_FILE);
        Ok(db_path.exists())
    }

    pub fn db_has_pending_migrations(&self) -> Result<bool> {
        self.with_db(|db| {
            db.conn.has_pending_migration(MIGRATIONS).map_err(|e| {
                HmsDbError::MigrationError(format!("Unable to determine pending migrations: {}", e))
            })
        })
    }

    pub fn run_pending_migrations(&self) -> Result<()> {
        self.with_db(|db| match db.conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(e) => Err(HmsDbError::MigrationError(format!(
                "Unable to run pending migrations: {}",
                e
            ))),
        })
    }

    pub fn with_db<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut HmsDb) -> Result<R>,
    {
        let database_path = self.app_dir_client.get_app_dir_path()?.join(DB_FILE);
        let mut conn = SqliteConnection::establish(&database_path.to_string_lossy())?;

        conn.transaction(|conn| {
            let mut db = HmsDb { conn };
            f(&mut db)
        })
    }
}
