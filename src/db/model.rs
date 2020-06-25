//! Model of db entities

use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub avi_location: Option<String>,
    pub hashed_pwd: String
}