// WebSocket server that sends structure updates to connected clients
// Run with: cargo run --bin server

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: SocketAddr = "127.0.0.1:9001".parse()?;
    let listener = TcpListener::bind(&addr).await?;
    println!("WebSocket server listening on: ws://{}", addr);

    while let Ok((stream, peer)) = listener.accept().await {
        println!("New client connected: {}", peer);
        tokio::spawn(handle_connection(stream, peer));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, peer: SocketAddr) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            println!("Error during WebSocket handshake with {}: {}", peer, e);
            return;
        }
    };

    println!("WebSocket connection established: {}", peer);

    let (mut write, mut read) = ws_stream.split();

    // Spawn a task to send periodic structure updates
    let send_task = tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(2));
        let mut angle = 0.0_f32;

        loop {
            interval.tick().await;

            // Generate rotating water molecule structure
            angle += 0.5;
            let distance = 0.757;

            let structure = json!({
                "atoms": [
                    {
                        "element": "O",
                        "x": 0.0,
                        "y": 0.0,
                        "z": 0.0
                    },
                    {
                        "element": "H",
                        "x": distance * angle.cos(),
                        "y": 0.587,
                        "z": distance * angle.sin()
                    },
                    {
                        "element": "H",
                        "x": -distance * angle.cos(),
                        "y": 0.587,
                        "z": -distance * angle.sin()
                    }
                ]
            });

            let msg = Message::Text(structure.to_string());
            if write.send(msg).await.is_err() {
                println!("Client {} disconnected", peer);
                break;
            }

            println!("Sent structure update to {}", peer);
        }
    });

    // Handle incoming messages (mostly for connection management)
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Close(_)) => {
                    println!("Client {} sent close frame", peer);
                    break;
                }
                Ok(Message::Ping(_)) => {
                    println!("Received ping from {}", peer);
                    // Pong will be sent automatically by tungstenite
                }
                Err(e) => {
                    println!("Error receiving message from {}: {}", peer, e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    println!("Connection closed: {}", peer);
}
