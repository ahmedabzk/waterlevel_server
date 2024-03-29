use axum::debug_handler;
use axum::extract::{Json, State};
use axum::http::HeaderMap;
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::app_state::AppState;
use crate::errors::custom_errors::CustomErrors;
use crate::models::statsmodel::NewStats;
use crate::utilis::jwt::verify_token;
use crate::utilis::token_wrapper::TokenWrapper;

#[debug_handler(state = AppState)]
pub async fn post_stats(
    State(db): State<PgPool>,
    State(token_secret): State<TokenWrapper>,
    header: HeaderMap,
    Json(new_stats): Json<NewStats>,
) -> Result<Json<Value>, CustomErrors> {
    let auth_header = header
        .get("x-auth-token")
        .ok_or(CustomErrors::Unauthorized)?
        .to_str()
        .map_err(|_| CustomErrors::InternalServerError)?;

    let user = verify_token(&token_secret.0, auth_header, &db)
        .await?
        .ok_or(CustomErrors::Unauthorized)?;

    let user_id = user.id;

    sqlx::query(
        "INSERT INTO stats (user_id, chlorine_level, ph, turbidity, water_level) values ($1, $2, $3, $4, $5)")
        .bind(user_id)
        .bind(new_stats.chlorine_level)
        .bind(new_stats.ph)
        .bind(new_stats.turbidity)
        .bind(new_stats.water_level)
        .execute(&db)
        .await?;

    Ok(Json(json!("created successfully")))
}
