use clap::Parser;
use migration::{Migrator, MigratorTrait};
use sea_orm::DbErr;
use server::app::AppBuilder;
use tracing::{debug, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "only fresh the database, not run the app")]
    pub fresh: bool,
}

#[tokio::main]
async fn main() {
    set_env();
    set_log();

    let args = Args::parse();
    if args.fresh {
        debug!("fresh");
        fresh_db().await.unwrap();
    } else {
        info!("run app");
        run_app().await;
    }
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
        .set_address(dotenv::var("ADDRESS").expect("cannot find ADDRESS"))
        .set_cors(dotenv::var("CORS").expect("connat find CORS"));

    builder.build().run().await;
}

async fn fresh_db() -> Result<(), DbErr> {
    let db = server::db::DbHelper::get_connection().await.unwrap();
    Migrator::fresh(&db).await?;
    db.close().await
}
