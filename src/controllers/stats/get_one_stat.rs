use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{errors::custom_errors::CustomErrors, models::statsmodel::ResponseStats};

pub async fn get_stat_by_id(
    State(db): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, CustomErrors> {
    let row = sqlx::query("SELECT * FROM stats where id=$1")
        .bind(id)
        .fetch_one(&db)
        .await?;

    let stat = ResponseStats::from_row(&row).expect("stat should be here");

    Ok(Json(json!(stat)))
}
