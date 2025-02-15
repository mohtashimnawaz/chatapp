use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use tokio::sync::broadcast;
use std::sync::Arc;
use crate::db::get_db;
use crate::models::Message as ChatMessage;
use mongodb::bson::doc;
use chrono::Utc;

pub async fn ws_handler(ws: WebSocketUpgrade, tx: Arc<broadcast::Sender<String>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: Arc<broadcast::Sender<String>>) {
    let mut rx = tx.subscribe();
    let db = get_db().await;
    let messages_collection = db.collection::<ChatMessage>("messages");

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            println!("Received message: {}", text);

            // Save to MongoDB
            let new_msg = ChatMessage {
                id: None,
                sender: "User".to_string(),
                content: text.clone(),
                timestamp: Utc::now().timestamp(),
            };

            messages_collection.insert_one(new_msg, None).await.unwrap();

            // Broadcast message
            let _ = tx.send(text);
        }
    }
}
