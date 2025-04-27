use crate::config::Config;
use crate::error::{Error, Result};
use crate::models::{MarketUpdate, SubscriptionMessage};
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, warn};
use url::Url;

fn create_market_subscription(asset_ids_to_subscribe: Vec<String>) -> Result<String> {
    let subscription_msg = SubscriptionMessage {
        auth: None,
        kind: "market".to_string(),
        assets_ids: asset_ids_to_subscribe,
        markets: Vec::new(),
    };

    serde_json::to_string(&subscription_msg).map_err(Error::Json)
}

pub async fn connect_and_stream_data(config: &Config) -> Result<()> {
    let url_str = &config.websocket_url;
    let url = Url::parse(url_str)?;

    info!("Connecting to Websocket: {}", url);

    // establish connection
    let (ws_stream, response) = connect_async(url_str).await?;

    info!("Websocket connected successfully!");
    debug!("Connection response: {:?}", response);

    // split stream into sender and receiver
    let (mut write, mut read) = ws_stream.split();

    // TODO: replace placeholders with ACTUAL Polymarket Token Ids
    let assets_to_subscribe = vec![
        "71321045679252212594626385532706912750332728571942532289631379312455583992563".to_string(), // bullcrap token Ids
        "52114319501245915516055106046884209969926127482827954674443846427813813222426".to_string(),
    ];

    if assets_to_subscribe.is_empty() {
        warn!("No asset IDs provided for subscription. Sending empty subscription.");
    }

    let json_string = create_market_subscription(assets_to_subscribe)?;

    write
        .send(Message::Text(json_string.clone().into()))
        .await?;

    info!("Sent subscription message: {}", json_string);

    // --- End subscription ---

    // --- Add Client-Side Ping Task --

    let (ping_tx, mut ping_rx) = tokio::sync::mpsc::channel::<Message>(1);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(50));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    debug!("Sending client PING");
                    if ping_tx.send(Message::Text("PING".to_string().into())).await.is_err() {
                        warn!("WebSocket connection closed, stopping ping task.");
                        break;
                    }
                }

                _ = ping_tx.closed() => {
                        warn!("Ping channel closed.");
                        break;
                    }
            }
        }
    });
    // --- End Client-Side Ping Task

    // --- Main Message Handling Loop ---
    loop {
        tokio::select! {
            maybe_message = read.next() => {
                match maybe_message {
                    Some(Ok(message)) => {
                        match message {
                            Message::Text(text) => {
                                debug!("Received Text message: {}", text);
                            // TODO: add parsing to response else print raw text

                            }
                            Message::Binary(bin) => {
                                debug!("Received Binary message: {} bytes", bin.len());
                            }
                            Message::Ping(ping_data) => {
                                debug!("Received Server Ping: {:?}", ping_data);
                            }
                            Message::Pong(pong_data) => {
                                debug!("Received Server Pong: {:?}", pong_data);
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
                    Some(Err(e)) => {
                        error!("Error reading from WebSocket: {}", e);
                        break;
                    }
                    None => {
                        info!("WebSocket stream ended.");
                        break;
                    }
                }
            }

            // Send client pings received from the ping task
            Some(ping_msg) = ping_rx.recv() => {
                if write.send(ping_msg).await.is_err() {
                    error!("Failed to send client PING to WebSocket.");
                    break;
                }
            }
        }
    }
    // --- End Main Message Handling Loop ---

    info!("WebSocket connection closed.");
    Ok(())

    // // handle incoming messages in a loop
    // while let Some(message_result) = read.next().await {
    //     match message_result {
    //         Ok(message) => {
    //             match message {
    //                 Message::Text(text) => {
    //                     debug!("Received Text message: {}", text);
    //
    //                     tracing::debug!("received {:?}", text);
    //                     // match serde_json::from_str::<MarketUpdate>(&text) {
    //                     //     Ok(parsed_data) => {
    //                     //         info!("Parsed Market Update: {:?}", parsed_data);
    //                     //     }
    //                     //     Err(e) => {
    //                     //         warn!("Failed to dserialize message: {}. Raw text: {}", e, text);
    //                     //     }
    //                     // }
    //                 }
    //                 Message::Binary(bin) => {
    //                     debug!("Received Binary message: {} bytes", bin.len());
    //                 }
    //                 Message::Ping(_ping_data) => {
    //                     debug!("Received Ping");
    //                 }
    //                 Message::Pong(pong_data) => {
    //                     debug!("Received Pong: {:?}", pong_data);
    //                 }
    //                 Message::Close(close_frame) => {
    //                     info!("Received Close frame: {:?}", close_frame);
    //                     break;
    //                 }
    //                 Message::Frame(_) => {
    //                     debug!("Received unspecified Frame");
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             error!("Error reading from WebSocket: {}", e);
    //             break;
    //         }
    //     }
    // }
    //
    // info!("Websocket connection closed.");
}
