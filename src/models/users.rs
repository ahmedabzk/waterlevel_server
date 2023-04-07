
use chrono;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow
};

use uuid::Uuid;

#[derive(Debug,Serialize,FromRow,Deserialize)]
pub struct User {
    pub id: Uuid::new_v4(),
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub create_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

let id = Uuid::new_v4();

