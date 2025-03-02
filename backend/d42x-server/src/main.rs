use clap::Parser;
use d42x_server::{app::AppBuilder, config};
use migration::{Migrator, MigratorTrait};
use sea_orm::DbErr;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "only fresh the database, not run the app")]
    pub fresh_db: bool,
    #[arg(short, long, help = "migrate database while app launcing")]
    pub migrate_db: bool,
}

#[tokio::main]
async fn main() {
    set_env();
    set_log();

    let args = Args::parse();
    if args.fresh_db {
        debug!("fresh");
        fresh_db().await.unwrap();
        return;
    }

    if args.migrate_db {
        info!("migrate_db");
        migrate_db().await.unwrap();
    }

    info!("run app");
    run_app().await;
}

fn set_env() {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.local").unwrap();
    }
}

fn set_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}

async fn run_app() {
    let mut builder = AppBuilder::new();
    builder
        .set_address(config::ADDRESS.to_string())
        .set_cors(config::CORS.to_string());

    builder.build().run().await;
}

async fn fresh_db() -> Result<(), DbErr> {
    let db = d42x_server::db::DbHelper::get_connection().await.unwrap();
    Migrator::fresh(&db).await?;
    db.close().await
}

async fn migrate_db() -> Result<(), DbErr> {
    let db = d42x_server::db::DbHelper::get_connection().await.unwrap();
    Migrator::up(&db, None).await?;
    db.close().await
}
