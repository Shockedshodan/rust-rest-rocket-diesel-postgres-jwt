pub mod auth;
pub mod model;

use crate::db::PostgresDatabaseConnection;
use crate::RocketBuild;
use auth::create_token;
use auth::Username;
use model::User;
use rocket::post;
use rocket::serde::json::{json, Json};
use rocket::{self, http::Status};
use rocket_dyn_templates::handlebars::JsonValue;

#[derive(Serialize, Deserialize)]
struct Credentials {
    username: String,
    password: String,
}

#[get("/")]
async fn list(connection: PostgresDatabaseConnection) -> Json<Vec<User>> {
    Json(connection.run(|c| User::list(c)).await)
}

#[get("/<id>")]
async fn get(id: i32, connection: PostgresDatabaseConnection) -> Json<User> {
    println!("Trying to get user with id {}", id);
    Json(connection.run(move |c| User::get(id, c)).await.unwrap())
}

#[get("/")]
async fn read(
    username: Username,
    connection: PostgresDatabaseConnection,
) -> Result<Json<User>, Status> {
    connection
        .run(move |c| User::by_name(&username.0[..], c))
        .await
        .map(|user| Json(user))
        .ok_or(Status::NotFound)
}

#[get("/", rank = 2)]
fn read_error() -> Json<JsonValue> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[post("/", data = "<credentials>")]
async fn login(
    credentials: Json<Credentials>,
    connection: PostgresDatabaseConnection,
) -> Result<Json<String>, Status> {
    match connection
        .run(move |c| {
            User::by_name_and_password(&credentials.username[..], &credentials.password[..], c)
        })
        .await
    {
        Some(user) => {
            let token = create_token(&user.name[..]);
            Ok(Json(token))
        }
        None => Err(Status::new(400)),
    }
}

pub fn mount(rocket: RocketBuild) -> RocketBuild {
    rocket
        .mount("/user", routes![read, read_error, get])
        .mount("/users", routes![list])
        .mount("/auth", routes![login])
}
