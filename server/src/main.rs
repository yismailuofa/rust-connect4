use std::collections::HashMap;

use argon2::{hash_encoded, verify_encoded};
use client::{ConnectGame, Leaderboard, User};
use rocket::{futures::TryStreamExt, http::Status, serde::json::Json};
use rocket_db_pools::{
    mongodb::{self, bson::doc, Collection},
    Connection, Database,
};

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("mongodb_main")] // same as DB_NAME
struct Db(mongodb::Client);

#[post("/create", data = "<game>")]
async fn create_game(db: Connection<Db>, game: Json<ConnectGame>) -> Result<(), Status> {
    let collection: Collection<ConnectGame> = db.database("mongodb_main").collection("games");

    let result = collection.insert_one(game.into_inner(), None).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/all")]
async fn all_games(db: Connection<Db>) -> Result<Json<Vec<ConnectGame>>, Status> {
    let collection: Collection<ConnectGame> = db.database("mongodb_main").collection("games");

    let cursor = collection
        .find(None, None)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let games = cursor
        .try_collect()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(games))
}

#[get("/login", data = "<user_payload>")]
async fn login(db: Connection<Db>, user_payload: Json<User>) -> Result<(), Status> {
    let collection: Collection<User> = db.database("mongodb_main").collection("users");

    let result = collection
        .find_one(Some(doc! {"username": &user_payload.username}), None)
        .await;

    match result {
        Ok(user) => match user {
            Some(user) => match verify_encoded(&user.password, user_payload.password.as_bytes()) {
                Ok(true) => Ok(()),
                Ok(false) => Err(Status::Unauthorized),
                Err(_) => Err(Status::InternalServerError),
            },
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/register", data = "<user_payload>")]
async fn register(db: Connection<Db>, user_payload: Json<User>) -> Result<(), Status> {
    let collection: Collection<User> = db.database("mongodb_main").collection("users");

    let result = collection
        .find_one(Some(doc! {"username": &user_payload.username}), None)
        .await;

    let hashed_user = User {
        username: user_payload.username.clone(),
        password: hash_encoded(
            user_payload.password.as_bytes(),
            b"supercalifragilisticexpialidocious",
            &argon2::Config::default(),
        )
        .unwrap(),
    };

    match result {
        Ok(user) => match user {
            Some(_) => Err(Status::Conflict),
            None => {
                let result = collection.insert_one(hashed_user, None).await;

                match result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Status::InternalServerError),
                }
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/")]
async fn leaderboard(db: Connection<Db>) -> Result<Json<Vec<Leaderboard>>, Status> {
    // Returns the top users grouped by their games
    let collection: Collection<ConnectGame> = db.database("mongodb_main").collection("games");

    let mut cursor = collection
        .find(None, None)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut leaderboard = HashMap::new();

    while let Some(game) = cursor
        .try_next()
        .await
        .map_err(|_| Status::InternalServerError)?
    {
        let loser = if game.player1 == game.winner {
            game.player2
        } else {
            game.player1
        };

        let mut entry = Leaderboard {
            username: game.winner.clone(),
            wins: 0,
            losses: 0,
        };

        let winner = leaderboard.entry(game.winner).or_insert(entry);

        winner.wins += 1;

        entry = Leaderboard {
            username: loser.clone(),
            wins: 0,
            losses: 0,
        };

        let loser = leaderboard.entry(loser).or_insert(entry);

        loser.losses += 1;
    }

    let mut leaderboard: Vec<Leaderboard> = leaderboard.into_iter().map(|(_, v)| v).collect();

    leaderboard.sort_by(|a, b| b.wins.cmp(&a.wins));

    Ok(Json(leaderboard))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/games", routes![create_game, all_games])
        .mount("/users", routes![login, register])
        .mount("/leaderboard", routes![leaderboard])
}
