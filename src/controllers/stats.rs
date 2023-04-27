
use axum::debug_handler;
use axum::extract::{State, Json};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use serde_json::{Value, json};
use sqlx::PgPool;


use crate::app_state::AppState;
use crate::models::statsmodel::NewStats;
use crate::errors::custom_errors::CustomErrors;
use crate::utilis::jwt::verify_token;
use crate::utilis::token_wrapper::TokenWrapper;


#[debug_handler(state = AppState)]
pub async fn post_stats(
    State(db): State<PgPool>,
    State(token_secret): State<TokenWrapper>,
    header: HeaderMap,
    Json(new_stats): Json<NewStats>,
) -> Result<Json<Value>, CustomErrors> {
    
    let auth_header = if let Some(token) = header.get("x-auth-token"){
        token.to_str().map_err(|err|{
                println!("error extracting token from header {:?}", err);
                CustomErrors::InternalServerError
            })?
    }else{
        return Err(CustomErrors::Unauthorized);
    };

    let user = verify_token(&token_secret.0, auth_header, &db)
        .await
        .unwrap().ok_or(CustomErrors::Unauthorized)?;

    let user_id = user.id;
    
    sqlx::query(
        "INSERT INTO stats (user_id, chlorine_level, ph, turbidity, water_level) values ($1, $2, $3, $4, $5)")
        .bind(user_id)
        .bind(new_stats.chlorine_level)
        .bind(new_stats.ph)
        .bind(new_stats.turbidity)
        .bind(new_stats.water_level)
        .execute(&db)
        .await
        .map_err(|_| CustomErrors::InternalServerError)?;

    Ok(Json(json!("created successfully")))
}