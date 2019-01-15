use ::actix::{Actor, SyncContext};
use ::diesel::pg::PgConnection;
use ::diesel::r2d2::{ConnectionManager, Pool};
use ::uuid::Uuid;
use ::chrono::{NaiveDateTime, Local};
use std::convert::From;

use crate::schema::{users,invitations};


pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);


impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}




#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct UserInsert {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
}


impl UserInsert {

    pub fn with_details(name: String, email: String, password: String) -> Self {
        Self {
            name,
            email,
            password,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
           email: user.email
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="invitations"]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}