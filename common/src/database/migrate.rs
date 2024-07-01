use crate::cli::MigrationCommands;
use crate::database::handle_panic::handle_panic;
use migration::{Migrator, MigratorTrait};
use sea_orm::DbConn;

pub async fn migrate(cli: MigrationCommands, db: &DbConn) {
    let result = match cli {
        MigrationCommands::Fresh => Migrator::fresh(db).await,
        MigrationCommands::Refresh => Migrator::refresh(db).await,
        MigrationCommands::Reset => Migrator::reset(db).await,
        MigrationCommands::Status => Migrator::status(db).await,
        MigrationCommands::Up { num } => Migrator::up(db, num).await,
        MigrationCommands::Down { num } => Migrator::down(db, Some(num)).await,
    };
    if let Err(e) = result {
        handle_panic(e)
    }
    println!("数据迁移完成")
}
