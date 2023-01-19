use crate::schema::posts;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub user_id: i32,
}

#[derive(AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost<'a> {
    pub title: Option<&'a str>,
    pub body: Option<&'a str>,
}

impl Post {
    pub fn find_by_user_id(user_id: i32, connection: &PgConnection) -> Vec<Post> {
        posts::table
            .filter(posts::user_id.eq(user_id))
            .limit(100)
            .load::<Post>(connection)
            .expect("Error when fetching posts")
    }

    pub fn find_by_id(id: i32, connection: &PgConnection) -> Option<Post> {
        posts::table
            .find(id)
            .first(connection)
            .optional()
            .expect("Post not found")
    }

    pub fn insert(new_post: &NewPost, connection: &PgConnection) -> Result<(), Error> {
        diesel::insert_into(posts::table)
            .values(new_post)
            .execute(connection)
            .map(|_| ())
            .map_err(|_| Error::NotFound)
    }

    pub fn update(
        update_post: &UpdatePost,
        post_id: i32,
        user_id: i32,
        connection: &PgConnection,
    ) -> Result<(), Error> {
        diesel::update(posts::table)
            .filter(posts::id.eq(post_id))
            .filter(posts::user_id.eq(user_id))
            .set(update_post)
            .get_result::<(i32, String, String, i32)>(connection)
            .map(|_| ())
            .map_err(|_| Error::NotFound)
    }

    pub fn delete(post_id: i32, user_id: i32, connection: &PgConnection) -> Result<(), Error> {
        match diesel::delete(posts::table)
            .filter(posts::id.eq(post_id))
            .filter(posts::user_id.eq(user_id))
            .execute(connection)
        {
            Ok(1) => Ok(()),
            Ok(_) => Err(Error::NotFound),
            Err(error) => Err(error),
        }
    }
}
