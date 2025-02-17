#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.local").unwrap();
    }

    env_logger::init();

    println!("Hello, world!");
}
