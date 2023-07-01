use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

struct AppState {
    users: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    kind: String,
    author: Option<String>,
    content: String,
}

impl ChatMessage {
    fn json(kind: &str, author: Option<&str>, content: &str) -> String {
        serde_json::to_string(&Self {
            kind: kind.to_owned(),
            author: author.map(|a| a.to_owned()),
            content: content.to_owned(),
        })
        .unwrap()
    }
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "build")] static_folder: PathBuf,
) -> shuttle_axum::ShuttleAxum {
    let users = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { users, tx });

    let app = Router::new()
        .fallback_service(get_service(ServeDir::new(static_folder)))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    Ok(app.into())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // Websocket sender and receiver
    let (mut sender, mut receiver) = stream.split();

    let mut username = String::new();

    // Loop until a text message is found
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            set_username(&state, &mut username, &name);

            // If the username was successfully assigned, break out of the loop, else let the client know the username is taken
            if !username.is_empty() {
                break;
            } else {
                sender
                    .send(Message::Text(ChatMessage::json(
                        "server",
                        None,
                        "Username already exists.",
                    )))
                    .await
                    .unwrap();

                return;
            }
        }
    }

    // Subscribe before sending the join message
    let mut rx = state.tx.subscribe();

    state
        .tx
        .send(ChatMessage::json(
            "server",
            None,
            &format!("{username} joined the chat."),
        ))
        .unwrap();

    // Listen to broadcast messages and send messages via websocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if sender.send(Message::Text(message)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();

    // Listen via websocket and send messages to broadcast subscribers
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // tx.send(format!("{name}: {text}")).unwrap();
            tx.send(ChatMessage::json("message", Some(&name), &text))
                .unwrap();
        }
    });

    // If either of the tasks is completed, abort the other
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // state.tx.send(format!("{username} left the chat.")).unwrap();
    state
        .tx
        .send(ChatMessage::json(
            "server",
            None,
            &format!("{username} left the chat."),
        ))
        .unwrap();
    state.users.lock().unwrap().remove(&username);
}

fn set_username(state: &AppState, string: &mut String, name: &str) {
    let mut users = state.users.lock().unwrap();

    if !users.contains(name) {
        users.insert(name.to_owned());
        string.push_str(name);
    }
}
