/*
 * Socket Server - A WebSocket server implementation in Rust
 * Copyright (C) 2024 Paul Werner (paulpwo)
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 * 
 * Repository: https://github.com/paulpwo/sockets_server_rust
 * Contact: https://github.com/paulpwo
 */

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::SocketRef,
    SocketIo,
};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use sysinfo::System;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{CorsLayer, Any};
use tracing::info;

#[derive(Clone)]
struct AppState {
    connected_clients: Arc<Mutex<HashMap<String, Instant>>>,
    system: Arc<Mutex<System>>,
    start_time: Instant,
    io: SocketIo,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    uptime: u64,
    memory: MemoryInfo,
    connections: usize,
}

#[derive(Serialize)]
struct MemoryInfo {
    rss: String,
    heap_used: String,
    heap_total: String,
}

#[derive(Serialize)]
struct MetricsResponse {
    connected_clients: usize,
    memory_usage: MemoryUsage,
    uptime: u64,
    timestamp: u64,
}

#[derive(Serialize)]
struct MemoryUsage {
    rss: u64,
    heap_used: u64,
    heap_total: u64,
}

#[derive(Deserialize)]
struct SendEventRequest {
    channel: Option<String>,
    channels: Option<Vec<String>>,
    payload: serde_json::Value,
}

#[derive(Serialize)]
struct SendEventResponse {
    status: String,
    events_sent: usize,
    duration: String,
}

// Socket.IO event handlers
fn on_connect(socket: SocketRef, state: Arc<AppState>) {
    let client_id = socket.id.to_string();
    info!("Cliente Socket.IO {} conectado", client_id);

    // Store client connection
    let state_clone = state.clone();
    let client_id_clone = client_id.clone();
    tokio::spawn(async move {
        state_clone.connected_clients.lock().await.insert(client_id_clone, Instant::now());
    });

    // Handle disconnect - clean up
    let state_disconnect = state.clone();
    let client_id_disconnect = client_id.clone();
    socket.on_disconnect(move || {
        info!("Cliente {} desconectado", client_id_disconnect);
        let state = state_disconnect.clone();
        let client_id = client_id_disconnect.clone();
        tokio::spawn(async move {
            // Remove from connected clients
            state.connected_clients.lock().await.remove(&client_id);
        });
    });
}

async fn health_handler(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let connected_clients = state.connected_clients.lock().await.len();
    let mut system = state.system.lock().await;
    system.refresh_memory();
    let memory = system.used_memory();
    let total_memory = system.total_memory();

    Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        uptime: state.start_time.elapsed().as_secs(),
        memory: MemoryInfo {
            rss: format!("{}MB", memory / 1024 / 1024),
            heap_used: format!("{}MB", memory / 1024 / 1024), // Approximate
            heap_total: format!("{}MB", total_memory / 1024 / 1024),
        },
        connections: connected_clients,
    })
}

async fn metrics_handler(State(state): State<Arc<AppState>>) -> Result<String, StatusCode> {
    let connected_clients = state.connected_clients.lock().await.len();
    let mut system = state.system.lock().await;
    system.refresh_memory();
    let memory = system.used_memory();

    let uptime = state.start_time.elapsed().as_secs();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let response = MetricsResponse {
        connected_clients,
        memory_usage: MemoryUsage {
            rss: memory,
            heap_used: memory, // Approximate
            heap_total: system.total_memory(),
        },
        uptime,
        timestamp,
    };

    serde_json::to_string(&response).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "WebSocket Server",
        "version": "1.0.0",
        "connectedClients": 0 // Will be updated
    }))
}

async fn send_websocket_event_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<SendEventRequest>,
) -> Result<Json<SendEventResponse>, StatusCode> {
    let start_time = Instant::now();

    let events_sent = match (request.channel, request.channels) {
        (Some(channel), None) => {
            // Single channel - clone the channel to avoid borrowing issues
            let channel_clone = channel.clone();
            let message = serde_json::json!({ "payload": request.payload });
            let _ = state.io.emit(channel_clone, message);
            info!("Evento enviado al canal '{}': {:?}", channel, request.payload);
            1
        }
        (None, Some(channels)) => {
            if channels.len() > 100 {
                return Err(StatusCode::BAD_REQUEST);
            }
            
            // Handle array of channels with array of payloads
            if let serde_json::Value::Array(payloads) = &request.payload {
                let length = std::cmp::min(channels.len(), payloads.len());
                for i in 0..length {
                    let channel = channels[i].clone();
                    let payload = payloads[i].clone();
                    let message = serde_json::json!({ "payload": payload });
                    let _ = state.io.emit(channel.clone(), message);
                    info!("Evento enviado al canal '{}': {:?}", channel, payload);
                }
                length
            } else {
                // Single payload for multiple channels
                let payload_clone = request.payload.clone();
                for channel in channels.iter() {
                    let channel_clone = channel.clone();
                    let message = serde_json::json!({ "payload": payload_clone });
                    let _ = state.io.emit(channel_clone, message);
                    info!("Evento enviado al canal '{}': {:?}", channel, payload_clone);
                }
                channels.len()
            }
        }
        _ => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let duration = start_time.elapsed().as_millis();
    info!("Evento enviado: {} eventos en {}ms", events_sent, duration);

    Ok(Json(SendEventResponse {
        status: "success".to_string(),
        events_sent,
        duration: format!("{}ms", duration),
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando servidor Socket.IO...");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new("info"))
        .init();

    // Create Socket.IO layer
    let (layer, io) = SocketIo::new_layer();

    let state = Arc::new(AppState {
        connected_clients: Arc::new(Mutex::new(HashMap::new())),
        system: Arc::new(Mutex::new(System::new())),
        start_time: Instant::now(),
        io: io.clone(),
    });

    // Register event handlers
    let state_clone = state.clone();
    io.ns("/", move |socket: SocketRef| {
        on_connect(socket, state_clone.clone());
    });

    // Build the app
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/", get(root_handler))
        .route("/SendWebsocketEvent", post(send_websocket_event_handler))
        .layer(
            ServiceBuilder::new()
                .layer(CompressionLayer::new()),
        )
        .layer(layer)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_credentials(false),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3030").await?;
    info!("Servidor Socket.IO iniciado en 0.0.0.0:3030");

    axum::serve(listener, app).await?;

    Ok(())
}
