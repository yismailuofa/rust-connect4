use chrono::{Datelike, Utc};
use std::collections::HashMap;

use argon2::{hash_encoded, verify_encoded};
use client::{ConnectGame, GameType, Leaderboard, User};
use mongodb::Client;
use rocket::{
    fairing::{Fairing, Info, Kind},
    futures::TryStreamExt,
    http::{Header, Status},
    serde::json::Json,
    Request, Response,
};
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

#[post("/login", data = "<user_payload>")]
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

#[get("/connect4")]
async fn connect4_leaderboard(db: Connection<Db>) -> Result<Json<Vec<Leaderboard>>, Status> {
    return fetch_leaderboard(db, "Connect4").await;
}

#[get("/tootandotto")]
async fn toototto_leaderboard(db: Connection<Db>) -> Result<Json<Vec<Leaderboard>>, Status> {
    return fetch_leaderboard(db, "TootAndOtto").await;
}

async fn fetch_leaderboard(
    db: Connection<Db>,
    game_type: &str,
) -> Result<Json<Vec<Leaderboard>>, Status> {
    let collection: Collection<ConnectGame> = db.database("mongodb_main").collection("games");

    let filter = doc! {"game_type": game_type};
    let mut cursor = collection
        .find(filter, None)
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

    leaderboard.sort_by(|a, b| {
        if a.wins == b.wins {
            a.losses.partial_cmp(&b.losses).unwrap()
        } else {
            b.wins.partial_cmp(&a.wins).unwrap()
        }
    });

    Ok(Json(leaderboard))
}

#[options("/<_..>")]
async fn options() -> Result<(), Status> {
    Ok(())
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://127.0.0.1:8080",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[rocket::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 2 && args[1] == "cli" {
        println!("Server Debugging CLI, make sure you have the server running in another terminal");

        let client: Client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .expect("Failed to initialize client.");

        let db = client.database("mongodb_main");

        let options = vec![
            "1. Create a game",
            "2. Get all games",
            "3. Login",
            "4. Register",
            "5. Exit",
        ];

        let mut input = String::new();

        loop {
            println!("Please select an option:");

            for option in &options {
                println!("{}", option);
            }

            input.clear();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let input: usize = input.trim().parse().unwrap();

            match input {
                1 => {
                    let games = db.collection::<ConnectGame>("games");

                    // Prompt for game fields
                    let mut input = String::new();
                    input.clear();

                    println!("Please enter the game type: 1. Connect4, 2. TootAndOtto");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    if (input.trim() != "1") && (input.trim() != "2") {
                        println!("Invalid game type");
                        continue;
                    }

                    let game_type: GameType = if input.trim() == "1" {
                        GameType::Connect4
                    } else {
                        GameType::TootAndOtto
                    };

                    input.clear();

                    println!("Please enter the player 1 username:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let player1 = input.trim().to_string();

                    input.clear();

                    println!("Please enter the player 2 username:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let player2 = input.trim().to_string();

                    input.clear();

                    println!("Please enter the winner username:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let winner = input.trim().to_string();

                    let date = Utc::now();

                    let formatted_date =
                        format!("{}-{}-{}", date.year(), date.month(), date.day(),);

                    let game = ConnectGame {
                        game_type,
                        player1,
                        player2,
                        winner,
                        date: formatted_date,
                    };

                    games.insert_one(game, None).await.unwrap();

                    println!("Game created");
                }
                2 => {
                    let games = db.collection::<ConnectGame>("games");

                    let mut cursor = games
                        .find(None, None)
                        .await
                        .expect("Failed to execute find.");

                    while let Some(result) = cursor.try_next().await.unwrap() {
                        println!("{:#?}", result);
                    }
                }
                3 => {
                    let users = db.collection::<User>("users");

                    let mut input = String::new();
                    input.clear();

                    println!("Please enter the username:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let username = input.trim().to_string();

                    input.clear();

                    println!("Please enter the password:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let password = input.trim().to_string();

                    let user = users
                        .find_one(
                            doc! {
                                "username": username,
                                "password": hash_encoded(
                                password.as_bytes(),
                                b"supercalifragilisticexpialidocious",
                                &argon2::Config::default(),
                            ).unwrap()
                            },
                            None,
                        )
                        .await
                        .unwrap();

                    if let Some(user) = user {
                        println!("User found: {:#?}", user);
                    } else {
                        println!("User not found");
                    }
                }
                4 => {
                    let users = db.collection::<User>("users");

                    let mut input = String::new();
                    input.clear();

                    println!("Please enter the username:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let username = input.trim().to_string();

                    input.clear();

                    println!("Please enter the password:");
                    std::io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");

                    let password = input.trim().to_string();

                    let user = User {
                        username,
                        password: hash_encoded(
                            password.as_bytes(),
                            b"supercalifragilisticexpialidocious",
                            &argon2::Config::default(),
                        )
                        .unwrap(),
                    };

                    users.insert_one(user, None).await.unwrap();

                    println!("User created");
                }
                5 => break,
                _ => println!("Invalid option"),
            }
        }
    } else {
        let _ = rocket::build()
            .attach(Db::init())
            .mount("/games", routes![create_game, all_games])
            .mount("/users", routes![login, register])
            .mount(
                "/leaderboard",
                routes![connect4_leaderboard, toototto_leaderboard],
            )
            .mount("/", routes![options])
            .attach(CORS)
            .launch()
            .await;
    }
}
