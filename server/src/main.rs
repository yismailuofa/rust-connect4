use rocket::{futures::TryStreamExt, http::Status, serde::json::Json};
use rocket_db_pools::{
    mongodb::{self, Collection},
    Connection, Database,
};
use server::ConnectGame;

#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("mongodb_main")] // same as DB_NAME
struct Db(mongodb::Client);

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/game", routes![all_games, create_game])
        .mount("/", routes![index]) // INDEX MUST BE LAST SINCE ROCKET WILL MATCH THE FIRST ROUTE IT FINDS
}
