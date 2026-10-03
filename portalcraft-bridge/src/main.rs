//! PortalCraft Bridge Daemon
//!
//! Connects Portal 2 (via named pipe on Windows) to Minecraft (via TCP).
//! Translates messages between the two transports.

use portalcraft_protocol::{decode, encode, Message, SessionId, Role};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio::time::interval;
use tracing::{debug, error, info, warn};

const PIPE_NAME: &str = r"\\.\pipe\portalcraft";
const TCP_ADDR: &str = "127.0.0.1:25565";
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const RECONNECT_DELAY: Duration = Duration::from_secs(2);
const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1 MB

#[derive(Debug)]
struct Peer {
    id: SessionId,
    role: Role,
    sender: mpsc::UnboundedSender<Message>,
    last_ping: Instant,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Starting PortalCraft Bridge Daemon");
    info!("Pipe: {}", PIPE_NAME);
    info!("TCP: {}", TCP_ADDR);

    let peers: Arc<Mutex<HashMap<SessionId, Peer>>> = Arc::new(Mutex::new(HashMap::new()));
    let (tx_bridge, mut rx_bridge) = mpsc::unbounded_channel::<(SessionId, Message)>();

    // Start named pipe server (Windows)
    let peers_pipe = peers.clone();
    let tx_pipe = tx_bridge.clone();
    tokio::spawn(async move {
        pipe_server(PIPE_NAME, peers_pipe, tx_pipe).await;
    });

    // Start TCP server for Minecraft
    let peers_tcp = peers.clone();
    let tx_tcp = tx_bridge.clone();
    tokio::spawn(async move {
        if let Err(e) = tcp_server(TCP_ADDR, peers_tcp, tx_tcp).await {
            error!("TCP server error: {}", e);
        }
    });

    // Heartbeat task
    let peers_hb = peers.clone();
    tokio::spawn(async move {
        let mut interval = interval(HEARTBEAT_INTERVAL);
        loop {
            interval.tick().await;
            let mut peers = peers_hb.lock().await;
            let now = Instant::now();
            peers.retain(|_, peer| {
                if now.duration_since(peer.last_ping) > Duration::from_secs(30) {
                    warn!("Peer {} timed out", peer.id);
                    false
                } else {
                    true
                }
            });
        }
    });

    // Message routing loop
    while let Some((from_id, msg)) = rx_bridge.recv().await {
        if let Err(e) = route_message(&peers, from_id, msg).await {
            error!("Routing error: {}", e);
        }
    }

    Ok(())
}

async fn pipe_server(
    _pipe_name: &str,
    _peers: Arc<Mutex<HashMap<SessionId, Peer>>>,
    _tx: mpsc::UnboundedSender<(SessionId, Message)>,
) {
    // TODO: Implement Windows named pipe server using tokio-named-pipes or windows-rs
    // For now, we'll just wait and log
    loop {
        info!("Named pipe server not yet implemented - waiting for Portal 2 connection on: {}", PIPE_NAME);
        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}

async fn tcp_server(
    addr: &str,
    peers: Arc<Mutex<HashMap<SessionId, Peer>>>,
    tx: mpsc::UnboundedSender<(SessionId, Message)>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("TCP server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                info!("New TCP connection from {}", addr);
                let peers = peers.clone();
                let tx = tx.clone();
                tokio::spawn(async move {
                    handle_tcp_client(stream, peers, tx).await;
                });
            }
            Err(e) => {
                error!("TCP accept error: {}", e);
            }
        }
    }
}

async fn handle_tcp_client(
    stream: TcpStream,
    peers: Arc<Mutex<HashMap<SessionId, Peer>>>,
    tx: mpsc::UnboundedSender<(SessionId, Message)>,
) {
    let (rd, wr) = stream.into_split();
    let mut reader = BufReader::new(rd);
    let mut writer = wr;
    let mut buf = vec![0u8; MAX_MESSAGE_SIZE];

    // Read hello message
    let len = match reader.read_u32_le().await {
        Ok(len) if len > 0 && len <= MAX_MESSAGE_SIZE as u32 => len as usize,
        Ok(_) => { warn!("Invalid message length"); return; }
        Err(e) => { debug!("Read length error: {}", e); return; }
    };

    buf.resize(len, 0);
    if let Err(e) = reader.read_exact(&mut buf).await {
        debug!("Read body error: {}", e);
        return;
    }

    let msg: Message = match decode(&buf) {
        Ok(m) => m,
        Err(e) => { warn!("Decode hello error: {}", e); return; }
    };

    let (session_id, role) = match msg {
        Message::Hello { protocol_version, session_id, role } => {
            if protocol_version != portalcraft_protocol::PROTOCOL_VERSION {
                let ack = Message::HelloAck {
                    session_id,
                    accepted: false,
                    reason: Some(format!("Protocol version mismatch: expected {}, got {}", portalcraft_protocol::PROTOCOL_VERSION, protocol_version)),
                };
                if let Ok(bytes) = encode(&ack) {
                    let _ = writer.write_u32_le(bytes.len() as u32).await;
                    let _ = writer.write_all(&bytes).await;
                }
                return;
            }
            (session_id, role)
        }
        _ => { warn!("Expected Hello message"); return; }
    };

    // Send ack
    let ack = Message::HelloAck {
        session_id,
        accepted: true,
        reason: None,
    };
    let ack_bytes = encode(&ack).unwrap();
    if writer.write_u32_le(ack_bytes.len() as u32).await.is_err() { return; }
    if writer.write_all(&ack_bytes).await.is_err() { return; }

    // Create outgoing channel for this peer
    let (out_tx, mut out_rx) = mpsc::unbounded_channel::<Message>();
    let out_tx_clone = out_tx.clone(); // Clone for the read loop
    
    // Register peer
    {
        let mut peers_lock = peers.lock().await;
        peers_lock.insert(session_id, Peer {
            id: session_id,
            role,
            sender: out_tx,
            last_ping: Instant::now(),
        });
    }
    info!("Peer registered: {} ({:?})", session_id, role);

    // Spawn writer task - handles ALL outgoing messages for this peer
    let mut writer_task_writer = writer;
    let writer_task = tokio::spawn(async move {
        while let Some(msg) = out_rx.recv().await {
            if let Ok(bytes) = encode(&msg) {
                if writer_task_writer.write_u32_le(bytes.len() as u32).await.is_err() { break; }
                if writer_task_writer.write_all(&bytes).await.is_err() { break; }
            }
        }
    });

    // Read loop - use cloned sender for responses
    loop {
        let len = match reader.read_u32_le().await {
            Ok(len) if len > 0 && len <= MAX_MESSAGE_SIZE as u32 => len as usize,
            Ok(_) => break,
            Err(_) => break,
        };

        buf.resize(len, 0);
        if reader.read_exact(&mut buf).await.is_err() { break; }

        let msg = match decode(&buf) {
            Ok(m) => m,
            Err(e) => { warn!("Decode error: {}", e); continue; }
        };

        // Update last ping
        {
            let mut peers_lock = peers.lock().await;
            if let Some(peer) = peers_lock.get_mut(&session_id) {
                peer.last_ping = Instant::now();
            }
        }

        // Handle messages
        match &msg {
            Message::Ping { timestamp } => {
                let pong = Message::Pong { timestamp: *timestamp };
                if out_tx_clone.send(pong).is_err() { break; }
            }
            Message::Disconnect { reason } => {
                info!("Peer {} disconnected: {}", session_id, reason);
                break;
            }
            _ => {
                // Forward to bridge
                if tx.send((session_id, msg)).is_err() { break; }
            }
        }
    }

    // Cleanup
    writer_task.abort();
    peers.lock().await.remove(&session_id);
    info!("Peer {} disconnected", session_id);
}

async fn route_message(
    peers: &Arc<Mutex<HashMap<SessionId, Peer>>>,
    from_id: SessionId,
    msg: Message,
) -> anyhow::Result<()> {
    let peers_lock = peers.lock().await;
    
    // Determine target roles based on message type and sender role
    let sender_role = peers_lock.get(&from_id).map(|p| p.role);
    
    let target_roles = match (&msg, sender_role) {
        // Portal 2 -> Bridge -> Minecraft
        (Message::PlaceBlock { .. }, Some(Role::Portal2Server)) => vec![Role::MinecraftClient],
        (Message::BreakBlock { .. }, Some(Role::Portal2Server)) => vec![Role::MinecraftClient],
        (Message::InventorySync { .. }, Some(Role::Portal2Server)) => vec![Role::MinecraftClient],
        (Message::PlayerState { .. }, Some(Role::Portal2Server)) => vec![Role::MinecraftClient, Role::Bridge],
        
        // Minecraft -> Bridge -> Portal 2
        (Message::PlaceBlock { .. }, Some(Role::MinecraftClient)) => vec![Role::Portal2Server],
        (Message::BreakBlock { .. }, Some(Role::MinecraftClient)) => vec![Role::Portal2Server],
        (Message::InventoryAction { .. }, Some(Role::MinecraftClient)) => vec![Role::Portal2Server],
        (Message::PlayerState { .. }, Some(Role::MinecraftClient)) => vec![Role::Portal2Server, Role::Bridge],
        
        // Bridge commands
        (Message::ChunkRequest { .. }, Some(Role::MinecraftClient)) => vec![Role::Portal2Server],
        (Message::ChunkData { .. }, Some(Role::Portal2Server)) => vec![Role::MinecraftClient],
        
        _ => vec![],
    };

    for (id, peer) in peers_lock.iter() {
        if target_roles.contains(&peer.role) && *id != from_id {
            if peer.sender.send(msg.clone()).is_err() {
                warn!("Failed to send to peer {}", id);
            }
        }
    }

    Ok(())
}