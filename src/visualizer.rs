use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, mpsc::Sender as StdSender, Arc, Mutex};
use std::time::Duration;
use tokio::sync::broadcast::{self};
use tokio::time;
use warp::ws::{Message, WebSocket};
use warp::Filter;

#[derive(Debug, Serialize, Clone)]
struct ConnectRspData {
    message: String,
    session_started: bool,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "cmd")]
pub enum FrontendMessage {
    ConnectRsp {
        data: ConnectRspData,
    },
    PacketNotify {
        data: Vec<PacketData>,
    },
    StartProxyRsp {
        data: bool,
    },
    StopProxyRsp {
        data: bool,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct PacketData {
    pub source: u8,        // 0=server, 1=client
    pub packetID: u16,
    pub protoName: String,
    pub object: serde_json::Value,
    pub time: u64,
    pub packet: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct VisualLog {
    pub source: u8,        // 0=server, 1=client
    pub packetID: u16,
    pub protoName: String,
    pub object: serde_json::Value,
    pub time: u64,
    pub packet: String,
}

static VISUALIZER_SENDER: Lazy<Arc<Mutex<Option<StdSender<VisualLog>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));
static SESSION_STARTED: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub fn init_visualizer_server() -> mpsc::Sender<VisualLog> {
    let (tx, rx) = mpsc::channel::<VisualLog>();
    *VISUALIZER_SENDER.lock().unwrap() = Some(tx.clone());

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let (btx, _) = broadcast::channel::<String>(1000);

            let btx_filter = warp::any().map({
                let btx = btx.clone();
                move || btx.clone()
            });

            let ws_route = warp::path("ws")
                .and(warp::ws())
                .and(warp::addr::remote())
                .and(btx_filter)
                .map(|ws: warp::ws::Ws, addr: Option<std::net::SocketAddr>, btx: broadcast::Sender<String>| {
                    ws.on_upgrade(move |socket| handle_socket(socket, btx.subscribe(), addr))
                });

            let (async_tx, mut async_rx) = tokio::sync::mpsc::channel::<VisualLog>(100);

            tokio::task::spawn_blocking(move || {
                while let Ok(log) = rx.recv() {
                    let _ = async_tx.blocking_send(log);
                }
            });

            tokio::spawn(async move {
                let mut packet_buffer = Vec::new();
                let mut interval = time::interval(Duration::from_millis(20));

                loop {
                    tokio::select! {
                        some_log = async_rx.recv() => {
                            if let Some(log) = some_log {
                                packet_buffer.push(log);
                            } else {
                                break;
                            }
                        }
                        _ = interval.tick() => {
                            if !packet_buffer.is_empty() {
                                let message = FrontendMessage::PacketNotify {
                                    data: packet_buffer.iter().map(|log| PacketData {
                                        source: log.source,
                                        packetID: log.packetID,
                                        protoName: log.protoName.clone(),
                                        object: log.object.clone(),
                                        time: log.time,
                                        packet: log.packet.clone(),
                                    }).collect(),
                                };
                                let _ = btx.send(serde_json::to_string(&message).unwrap());
                                packet_buffer.clear();
                            }
                        }
                    }
                }
            });

            warp::serve(ws_route)
                .run(([127, 0, 0, 1], 40510))
                .await;
        });
    });

    tx
}

async fn handle_socket(ws: WebSocket, mut stream: broadcast::Receiver<String>, addr: Option<std::net::SocketAddr>) {
    if let Some(addr) = addr {
        println!("New WebSocket connection from {}", addr);
    } else {
        println!("New WebSocket connection from unknown address");
    }
    let (mut tx, mut rx) = ws.split();

    loop {
        tokio::select! {
            result = rx.next() => {
                match result {
                    Some(Ok(msg)) => {
                        if let Ok(text_str) = msg.to_str() {
                            if let Ok(json) = serde_json::from_str::<Value>(text_str) {
                                match json.get("cmd").and_then(|v| v.as_str()) {
                                    Some("ConnectReq") => {
                                        let response = FrontendMessage::ConnectRsp {
                                            data: ConnectRspData {
                                                message: "connected".to_string(),
                                                session_started: SESSION_STARTED.load(Ordering::SeqCst),
                                            },
                                        };
                                        if tx.send(Message::text(serde_json::to_string(&response).unwrap())).await.is_err() {
                                            break;
                                        }
                                    }
                                    Some("StartProxyReq") => {
                                        SESSION_STARTED.store(true, Ordering::SeqCst);
                                        let response = FrontendMessage::StartProxyRsp {
                                            data: true,
                                        };
                                        if tx.send(Message::text(serde_json::to_string(&response).unwrap())).await.is_err() {
                                            break;
                                        }
                                    }
                                    Some("StopProxyReq") => {
                                        SESSION_STARTED.store(false, Ordering::SeqCst);
                                        let response = FrontendMessage::StopProxyRsp {
                                            data: true,
                                        };
                                        if tx.send(Message::text(serde_json::to_string(&response).unwrap())).await.is_err() {
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Some(Err(_)) => break,
                    None => break,
                }
            }
            result = stream.recv() => {
                match result {
                    Ok(msg) => {
                        if tx.send(Message::text(msg)).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }
}

pub fn send_visual_log(log: VisualLog) {
    if let Some(sender) = &*VISUALIZER_SENDER.lock().unwrap() {
        let _ = sender.send(log);
    }
}