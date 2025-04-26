mod config;
mod error;
mod models;
mod network;

use error::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("Starting Prediction Arbitrage Bot...");

    let config = config::load_config()?;
    info!("Configuration loaded sucessfully!");

    info!("Initializing WebSocket connection...");
    let connection_handle = tokio::spawn(async move {
        if let Err(e) = network::websocket::connect_and_stream_data(&config).await {
            eprintln!("Websocket connection error: {}", e);
            tracing::error!("Websocket task failed: {}", e);
        }
    });

    // keep the main thread alive until interrupted (Ctrl+C)
    // or until the connection task finished (e.g., due to error)
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Ctrl+C received, shutting down.");
        }
        _ = connection_handle => {
            info!("Websocket task finished.");
        }
    }
    Ok(())
}
