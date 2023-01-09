mod model;

use crate::db::PostgresDatabaseConnection;
use crate::user::auth::Username;
use crate::user::model::User;
use crate::RocketBuild;
use rocket::serde::json::Json;
use rocket::{self, http::Status};
use serde_derive::Deserialize;

use self::model::NewPost;
use self::model::Post;
use self::model::UpdatePost;

#[derive(Deserialize)]
struct NewPostRequest {
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct UpdatePostRequest {
    id: i32,
    title: Option<String>,
    body: Option<String>,
}

#[get("/")]
async fn list(username: Username, connection: PostgresDatabaseConnection) -> Json<Vec<Post>> {
    let user_id = connection
        .run(move |c| User::by_name(&username.0[..], c))
        .await
        .unwrap()
        .id;
    Json(
        connection
            .run(move |c| Post::find_by_user_id(user_id, c))
            .await,
    )
}

#[get("/<id>")]
async fn read(
    id: i32,
    _username: Username,
    connection: PostgresDatabaseConnection,
) -> Result<Json<Post>, Status> {
    connection
        .run(move |c| Post::find_by_id(id, c))
        .await
        .map(|post| Json(post))
        .ok_or_else(|| Status::NotFound)
}

#[post("/", data = "<post>")]
async fn create(
    post: Json<NewPostRequest>,
    username: Username,
    connection: PostgresDatabaseConnection,
) -> Result<Status, Status> {
    let user_id = connection
        .run(move |c| User::by_name(&username.0[..], c))
        .await
        .unwrap()
        .id;
    connection
        .run(move |c| {
            Post::insert(
                &NewPost {
                    title: &post.title[..],
                    body: &post.body[..],
                    user_id,
                },
                c,
            )
        })
        .await
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[put("/", data = "<post>")]
async fn update(
    post: Json<UpdatePostRequest>,
    username: Username,
    connection: PostgresDatabaseConnection,
) -> Result<Status, Status> {
    let user_id = connection
        .run(move |c| User::by_name(&username.0[..], c))
        .await
        .unwrap()
        .id;
    connection
        .run(move |c| {
            Post::update(
                &UpdatePost {
                    title: post.title.as_ref().map(|t| &t[..]),
                    body: post.body.as_ref().map(|t| &t[..]),
                },
                post.id,
                user_id,
                c,
            )
        })
        .await
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/<id>")]
async fn delete(
    id: i32,
    username: Username,
    connection: PostgresDatabaseConnection,
) -> Result<Status, Status> {
    let user_id = connection
        .run(move |c| User::by_name(&username.0[..], c))
        .await
        .unwrap()
        .id;
    connection
        .run(move |c| Post::delete(id, user_id, c))
        .await
        .map(|_| Status::Ok)
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: RocketBuild) -> RocketBuild {
    rocket
        .mount("/post", routes![read, create, update, delete])
        .mount("/posts", routes![list])
}
