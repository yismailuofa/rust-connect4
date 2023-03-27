use argon2::{hash_encoded, verify_encoded};
use rocket::{futures::TryStreamExt, http::Status, serde::json::Json};
use rocket_db_pools::{
    mongodb::{self, bson::doc, Collection},
    Connection, Database,
};
use server::{ConnectGame, User};

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

    let result = collection.find(None, None).await;

    match result {
        Ok(cursor) => {
            let games = cursor.try_collect::<Vec<ConnectGame>>().await;

            match games {
                Ok(games) => Ok(Json(games)),
                Err(_) => Err(Status::InternalServerError),
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/", data = "<user_payload>")]
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

#[post("/", data = "<user_payload>")]
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/game", routes![all_games, create_game])
        .mount("/login", routes![login])
        .mount("/register", routes![register])
}
