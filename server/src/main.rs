use rocket::{http::Status, serde::json::Json};
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

#[post("/game/create", data = "<game>")]
async fn create_game(db: Connection<Db>, game: Json<ConnectGame>) -> Result<(), Status> {
    let collection: Collection<ConnectGame> = db.database("mongodb_main").collection("games");

    let result = collection.insert_one(game.into_inner(), None).await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![create_game, index]) // INDEX MUST BE LAST SINCE ROCKET WILL MATCH THE FIRST ROUTE IT FINDS
}
