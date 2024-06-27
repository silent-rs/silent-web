use crate::cli::Cli;
use clap::Parser;
use commom::handle_panic;
use migration::sea_orm::{ConnectOptions, Database};

mod cli;
mod migrate;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let url = args
        .database_url
        .expect("Environment variable 'DATABASE_URL' not set");
    let schema = args.database_schema.unwrap_or_else(|| "public".to_owned());

    let connect_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .to_owned();
    let db = &Database::connect(connect_options)
        .await
        .expect("Fail to acquire database connection");
    // 迁移数据库
    if let Some(migrate_cli) = args.migrate {
        migrate::migrate(migrate_cli, db)
            .await
            .unwrap_or_else(handle_panic);
        return;
    }
    if args.server {
        // todo!("start server")
    }
}
