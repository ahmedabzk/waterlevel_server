use axum::debug_handler;
use axum::extract::{Json, State};
use axum::http::HeaderMap;
use serde_json::{json, Value};
use sqlx::{FromRow, PgPool};

use crate::app_state::AppState;
use crate::errors::custom_errors::CustomErrors;
use crate::models::statsmodel::{ResponseStats};
use crate::utilis::jwt::verify_token;
use crate::utilis::token_wrapper::TokenWrapper;

#[debug_handler(state = AppState)]
pub async fn get_all_stats(
    State(db): State<PgPool>,
    State(token_secret): State<TokenWrapper>,
    header: HeaderMap,
) -> Result<Json<Value>, CustomErrors> {


    let auth_header = header
        .get("x-auth-token")
        .ok_or(CustomErrors::Unauthorized)?
        .to_str()
        .map_err(|_| CustomErrors::InternalServerError)?;

    let user = verify_token(&token_secret.0, auth_header, &db)
        .await?
        .ok_or(CustomErrors::InvalidToken)?;

    let row = sqlx::query("SELECT * FROM stats where user_id=$1")
        .bind(user.id)
        .fetch_all(&db)
        .await?
        .iter()
        .map(|stats| ResponseStats::from_row(stats).expect("values should be there"))
        .collect::<Vec<_>>();

    Ok(Json(json!(row)))
}
