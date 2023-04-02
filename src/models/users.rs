
use serde::{Serialize, Deserialize};
use sqlx;

#[derive(Serialize, sqlx::FromRow, Deserialize)]
pub struct User{
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String
}

