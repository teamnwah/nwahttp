use crate::server_info::events::{FullPlayerEvent, PlayerPositionEvent, WebsocketEvent};
use crate::server_info::player_details::Player;
use futures_util::SinkExt;
use std::collections::HashMap;
use std::future::Future;
use std::os::raw::c_ushort;
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::{Mutex, RwLock};
use warp::ws::{Message, WebSocket};

#[derive(Default, Debug)]
pub struct ServerInfo {
    pub players: HashMap<c_ushort, Player>,
}

#[derive(Default, Debug)]
pub struct ServerLogic {
    pub web_sockets: HashMap<u64, WebSocket>,
}

type SyncMutex<T> = std::sync::Mutex<T>;

#[derive(Clone, Debug)]
pub struct ServerInfoHandle {
    pub info: Arc<RwLock<ServerInfo>>,
    pub logic: Arc<Mutex<ServerLogic>>,
    pub runtime: Arc<SyncMutex<Runtime>>,
    pub handle: Arc<Handle>,
}

impl ServerInfoHandle {
    pub fn new() -> Self {
        let runtime = Runtime::new().unwrap();
        let handle = runtime.handle().clone();

        ServerInfoHandle {
            info: Arc::new(RwLock::new(ServerInfo::default())),
            logic: Arc::new(Mutex::new(ServerLogic::default())),
            runtime: Arc::new(SyncMutex::new(runtime)),
            handle: Arc::new(handle),
        }
    }

    pub async fn publish_event(&self, event: WebsocketEvent) {
        let logic = self.logic.clone();

        self.handle.spawn(async move {
            let mut logic = logic.lock().await;
            let json = serde_json::to_string(&event).unwrap();

            let mut to_remove = vec![];
            for (id, web_socket) in &mut logic.web_sockets {
                if web_socket.send(Message::text(&json)).await.is_err() {
                    to_remove.push(*id);
                }
            }

            for id in to_remove {
                logic.web_sockets.remove(&id).map(|x| x.close());
            }
        });
    }

    pub async fn gui_action(&self, player_id: u16, _message_box_id: i32, _data: Option<&str>) {
        let mut info = self.info.write().await;
        if let Some(player) = info.players.get_mut(&player_id) {
            if !player.logged_in {
                player.logged_in = true;
                player.on_login();
            }
        }
    }

    pub async fn update_players(&self, low_freq: bool) {
        let mut info = self.info.write().await;
        for (_, player) in &mut info.players {
            if !player.logged_in {
                continue;
            }

            player.update();

            if low_freq {
                player.low_frequency_update();
            }
        }

        if info.players.len() == 0 {
            return;
        }

        let players = info.players.clone();
        self.publish_event(if low_freq {
            WebsocketEvent::FullPlayer(FullPlayerEvent {
                players: players
                    .values()
                    .filter(|p| p.logged_in)
                    .map(|p| p.clone())
                    .collect::<Vec<Player>>(),
            })
        } else {
            WebsocketEvent::PlayerPosition(PlayerPositionEvent {
                positions: players
                    .values()
                    .filter(|p| p.logged_in)
                    .map(|p| p.get_player_position())
                    .collect(),
            })
        })
        .await;
    }

    pub async fn add_player(&self, player_id: c_ushort) {
        let mut info = self.info.write().await;
        info.players.insert(player_id, Player::new(player_id));
    }

    pub async fn remove_player(&self, player: c_ushort) {
        let mut info = self.info.write().await;
        info.players.remove(&player);

        if info.players.len() == 0 {
            self.publish_event(WebsocketEvent::FullPlayer(FullPlayerEvent {
                players: vec![],
            }))
            .await;
        }
    }

    pub async fn get_players(&self) -> Vec<Player> {
        let info = self.info.read().await;

        info.players.iter().map(|p| (*p.1).clone()).collect()
    }

    pub async fn add_websocket(&self, ws: WebSocket) {
        let mut logic = self.logic.lock().await;
        let new_id = logic
            .web_sockets
            .keys()
            .max()
            .map(|x| x + 1)
            .unwrap_or_default();
        println!("Added websocket ({})", new_id);
        logic.web_sockets.insert(new_id, ws);
    }

    pub fn block_on<F: Future>(&mut self, task: F) -> F::Output {
        self.runtime.lock().unwrap().block_on(task)
    }
}
