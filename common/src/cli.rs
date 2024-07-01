use clap::{Parser, Subcommand};

pub fn init_args() -> Cli {
    dotenv::dotenv().expect("读取.env文件失败");
    Cli::parse()
}

#[derive(Parser)]
/// Silent Web 服务框架
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand, help = "数据库迁移")]
    pub migrate: Option<MigrationCommands>,
    #[arg(long, help = "启动 Web 服务", default_value_t = true)]
    pub server: bool,

    #[arg(
        global = true,
        short = 'u',
        long,
        env = "DATABASE_URL",
        help = "数据库链接"
    )]
    pub(crate) database_url: Option<String>,

    #[arg(
        global = true,
        short = 's',
        long,
        env = "DATABASE_SCHEMA",
        long_help = "数据库架构\n \
                    - 对于MySQL和SQLite，此参数将被忽略\n \
                    - 对于PostgreSQL，此参数是可选的，默认值为 \"public\"\n"
    )]
    pub(crate) database_schema: Option<String>,
}

#[derive(Subcommand)]
pub enum MigrationCommands {
    #[command(
        about = "从数据库中删除所有表，然后重新应用所有迁移",
        display_order = 30
    )]
    Fresh,
    #[command(about = "回滚所有应用的迁移，然后重新应用所有迁移", display_order = 40)]
    Refresh,
    #[command(about = "回滚所有应用的迁移", display_order = 50)]
    Reset,
    #[command(about = "检查所有迁移的状态", display_order = 60)]
    Status,
    #[command(about = "执行迁移", display_order = 70)]
    Up {
        #[arg(short, long, help = "迁移数量")]
        num: Option<u32>,
    },
    #[command(about = "迁移回滚", display_order = 80)]
    Down {
        #[arg(
            short,
            long,
            default_value = "1",
            help = "回滚数量",
            display_order = 90
        )]
        num: u32,
    },
}
