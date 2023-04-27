use chrono;
use serde::{Serialize, Deserialize};


use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Stats{
    #[serde(skip)]
    pub id: Uuid,
    #[serde(skip)]
    pub user_id: Uuid,
    pub chlorine_level: f32,
    pub ph: f32,
    pub turbidity: f32,
    pub water_level: f32,
    pub create_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewStats{
    pub chlorine_level: f32,
    pub ph: f32,
    pub turbidity: f32,
    pub water_level: f32,
}
