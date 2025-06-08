use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::Utc;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::{
    sync::{mpsc, Mutex},
    time::{sleep, Duration},
};

use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::types::{MessageCallback, MessageHandler};

const HEARTBEAT_INTERVAL: u64 = 20;
const RETRY_DELAY: u64 = 5;
const MAX_RETRY_ATTEMPTS: u32 = 10;
const MAX_RETRY_DELAY: u64 = 60;

async fn connect_websocket(
    wss_domain: &str,
) -> Result<(
    WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    mpsc::Sender<Message>,
    mpsc::Receiver<Message>,
)> {
    let ws_url = format!("wss://{}/v4/ws/usdt", wss_domain);
    let (ws_stream, _) = connect_async(&ws_url).await?;
    let (tx, rx) = mpsc::channel(100);
    Ok((ws_stream, tx, rx))
}

async fn subscribe_channel(
    write: &mut futures::stream::SplitSink<
        WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
        Message,
    >,
    interval: &str,
    symbol: &str,
) -> Result<()> {
    let subscribe_msg = json!({
        "time": Utc::now().timestamp(),
        "channel": "futures.candlesticks",
        "event": "subscribe",
        "payload": [interval, symbol]
    })
    .to_string();
    write
        .send(Message::Text(subscribe_msg))
        .await
        .context("【gate】 Failed to send subscribe message over WebSocket")?;
    Ok(())
}

pub async fn run_with_handler(wss_domain: &str, handler: Arc<dyn MessageHandler>) -> Result<()> {
    run_internal(wss_domain, Some(handler), None).await
}

pub async fn run_with_callback(wss_domain: &str, callback: MessageCallback) -> Result<()> {
    run_internal(wss_domain, None, Some(callback)).await
}

async fn run_internal(
    wss_domain: &str,
    handler: Option<Arc<dyn MessageHandler>>,
    callback: Option<MessageCallback>,
) -> Result<()> {
    println!("初始化 gate WebSocket...");
    let mut retry_count = 0;
    let mut retry_delay = RETRY_DELAY;

    let interval = "1m"; // 可参数化
    let symbol = "BTC_USD"; // 可参数化

    loop {
        match connect_websocket(wss_domain).await {
            Ok((ws_stream, tx, mut rx)) => {
                let (write_half, mut read) = ws_stream.split();
                let write = Arc::new(Mutex::new(write_half));

                retry_count = 0;
                retry_delay = RETRY_DELAY;

                // 初次订阅
                {
                    let mut write_guard = write.lock().await;
                    if let Err(e) = subscribe_channel(&mut *write_guard, interval, symbol).await {
                        println!("初次订阅失败: {:?}", e);
                        continue;
                    }
                }

                let tx_clone = tx.clone();
                let heartbeat_handle = tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(HEARTBEAT_INTERVAL)).await;
                        let ping =
                            json!({"op": "ping", "ping": Utc::now().timestamp()}).to_string();
                        if let Err(e) = tx_clone.send(Message::Text(ping)).await {
                            println!("发送心跳失败: {:?}", e);
                        }
                    }
                });

                let write_sender_clone = Arc::clone(&write);
                let sender_handle = tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        let mut write_guard = write_sender_clone.lock().await;
                        if let Err(e) = write_guard.send(msg).await {
                            println!("发送消息失败: {:?}", e);
                        }
                    }
                });

                let write_sub_clone = Arc::clone(&write);
                let subscribe_handle = tokio::spawn(async move {
                    loop {
                        sleep(Duration::from_secs(60)).await;
                        let mut write_guard = write_sub_clone.lock().await;
                        if let Err(e) = subscribe_channel(&mut *write_guard, interval, symbol).await
                        {
                            println!("定时订阅失败: {:?}", e);
                        }
                    }
                });

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(msg) if msg.is_text() => {
                            let text = msg.to_text().unwrap_or("");
                            if let Some(ref handler) = handler {
                                handler.handle(text).await;
                            }
                            if let Some(ref callback) = callback {
                                callback(text).await;
                            }
                        }
                        Ok(_) => {} // 忽略非文本消息
                        Err(e) => {
                            println!("WebSocket 接收错误: {:?}", e);
                            break;
                        }
                    }
                }

                heartbeat_handle.abort();
                sender_handle.abort();
                subscribe_handle.abort();
            }
            Err(e) => {
                println!("连接失败: {:?}", e);
            }
        }

        retry_count += 1;
        if retry_count >= MAX_RETRY_ATTEMPTS {
            println!("达到最大重试次数，退出");
            break;
        }

        println!("{} 秒后重试连接...", retry_delay);
        sleep(Duration::from_secs(retry_delay)).await;
        retry_delay = (retry_delay * 2).min(MAX_RETRY_DELAY);
    }

    Ok(())
}
