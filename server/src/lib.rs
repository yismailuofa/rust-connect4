use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ConnectGame {
    pub game_type: GameType,
    pub player1: String,
    pub player2: String,
    pub winner: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub enum GameType {
    Connect4,
    TootAndOtto,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub username: String,
    pub password: String,
}
