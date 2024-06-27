use migration::sea_orm::DbConn;
use migration::{Migrator, MigratorTrait};

use crate::cli::MigrationCommands;

pub async fn migrate(cli: MigrationCommands, db: &DbConn) -> anyhow::Result<()> {
    match cli {
        MigrationCommands::Fresh => {
            Migrator::fresh(db).await?;
        }
        MigrationCommands::Refresh => {
            Migrator::refresh(db).await?;
        }
        MigrationCommands::Reset => {
            Migrator::reset(db).await?;
        }
        MigrationCommands::Status => {
            Migrator::status(db).await?;
        }
        MigrationCommands::Up { num } => {
            Migrator::up(db, num).await?;
        }
        MigrationCommands::Down { num } => {
            Migrator::down(db, Some(num)).await?;
        }
    };
    Ok(())
}
