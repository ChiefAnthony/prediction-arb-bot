use crate::config::Config;
use crate::error::{Error, Result};
use crate::models::MarketUpdate;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tracing::{debug, error, info, warn};
use url::Url;

pub async fn connect_and_stream_data(config: &Config) -> Result<()> {
    let url_str = &config.websocket_url;
    let url = Url::parse(url_str)?;

    info!("Connecting to Websocket: {}", url);

    // establish connection
    let (ws_stream, response) = connect_async(url_str).await?;

    info!("Websocket connected successfully!");
    debug!("Connection response: {:?}", response);

    // split stream into sender and receiver
    let (mut _write, mut read) = ws_stream.split();

    // handle incoming messages in a loop
    while let Some(message_result) = read.next().await {
        match message_result {
            Ok(message) => {
                match message {
                    Message::Text(text) => {
                        debug!("Received Text message: {}", text);
                        // attempt to dserialize the message
                        match serde_json::from_str::<MarketUpdate>(&text) {
                            Ok(parsed_data) => {
                                info!("Parsed Market Update: {:?}", parsed_data);
                            }
                            Err(e) => {
                                warn!("Failed to dserialize message: {}. Raw text: {}", e, text);
                            }
                        }
                    }
                    Message::Binary(bin) => {
                        debug!("Received Binary message: {} bytes", bin.len());
                    }
                    Message::Ping(_ping_data) => {
                        debug!("Received Ping");
                    }
                    Message::Pong(pong_data) => {
                        debug!("Received Pong: {:?}", pong_data);
                    }
                    Message::Close(close_frame) => {
                        info!("Received Close frame: {:?}", close_frame);
                        break;
                    }
                    Message::Frame(_) => {
                        debug!("Received unspecified Frame");
                    }
                }
            }
            Err(e) => {
                error!("Error reading from WebSocket: {}", e);
                break;
            }
        }
    }

    info!("Websocket connection closed.");
    Ok(())
}
