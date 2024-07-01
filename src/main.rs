use api::get_routes;
use common::cli::init_args;
use common::database::{get_db_conn, migrate};
use common::server::{init_route, init_server};

#[tokio::main]
async fn main() {
    let args = init_args();

    let db = get_db_conn(&args).await;
    // 迁移数据库
    if let Some(migrate_cli) = args.migrate {
        migrate(migrate_cli, &db).await;
        return;
    }
    if args.server {
        // todo!("start server")
        init_server().serve(init_route(get_routes())).await
    }
}
