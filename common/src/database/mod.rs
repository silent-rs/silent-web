use crate::cli::Cli;
use sea_orm::{ConnectOptions, Database, DbConn};

mod handle_panic;
mod migrate;

pub use migrate::migrate;

pub async fn get_db_conn(args: &Cli) -> DbConn {
    let url = args
        .database_url
        .clone()
        .expect("Environment variable 'DATABASE_URL' not set");
    let schema = args
        .database_schema
        .clone()
        .unwrap_or_else(|| "public".to_owned());

    let connect_options = ConnectOptions::new(url)
        .set_schema_search_path(schema)
        .to_owned();
    Database::connect(connect_options)
        .await
        .expect("Fail to acquire database connection")
}
