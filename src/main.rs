use dotenvy::dotenv;
use tracing_subscriber::{self, fmt::init};

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    tracing::info!("Application started");
    println!("Hello World!");
}
