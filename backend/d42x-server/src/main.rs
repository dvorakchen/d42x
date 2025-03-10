use clap::Parser;
use d42x_server::{
    app::shared_data::{CategoryRepoSS, MemeRepoSS},
    business::{
        cache::MokaCache, category::gen_cate_repo::GenCategoryRepo,
        meme::gen_meme_repo::GenMemeRepo,
    },
    config,
    db::shared_db_helper::SharedDbHelper,
};
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
    build_run().await;
    // run_app().await;
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

async fn build_run() {
    let cate_repo = category_repo_shared_state();
    let meme_repo = meme_repo_shared_state();

    d42x_server::app::AppBuilder::new()
        .address(config::ADDRESS.to_string())
        .cors(config::CORS.to_string())
        .category_repo(cate_repo)
        .meme_repo(meme_repo)
        .aes_key(config::KEY.to_string())
        .aes_iv(config::IV.clone())
        .build()
        .await
        .run()
        .await;
}

fn category_repo_shared_state() -> CategoryRepoSS {
    let db = SharedDbHelper::new(config::DATABASE_URL.to_string());
    let cate_repo = GenCategoryRepo::with_cache(db, Some(MokaCache::new()));
    CategoryRepoSS::new(cate_repo)
}

fn meme_repo_shared_state() -> MemeRepoSS {
    let db = SharedDbHelper::new(config::DATABASE_URL.to_string());
    let meme_repo = GenMemeRepo::with_cache(db, Some(MokaCache::new()));
    MemeRepoSS::new(meme_repo)
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
