use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectGame {
    pub game_type: GameType,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GameType {
    Connect4,
    TootAndOtto,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Leaderboard {
    pub username: String,
    pub wins: u32,
    pub losses: u32,
}
