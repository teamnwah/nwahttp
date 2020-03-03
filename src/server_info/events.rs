use crate::server_info::player_details::Player;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum WebsocketEvent {
    FullPlayer(FullPlayerEvent),
    PlayerPosition(PlayerPositionEvent),
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FullPlayerEvent {
    pub players: Vec<Player>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPositionEvent {
    pub positions: Vec<PlayerPosition>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPosition {
    pub name: String,
    pub position: (f64, f64),
    pub rotation: f64,
    pub cell: String,
    pub is_outside: bool,
}
