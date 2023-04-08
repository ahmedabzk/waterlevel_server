use axum::{debug_handler, Extension, Json};
// use serde::{Deserialize, Serialize};
// use serde_json;

use sqlx::PgPool;
use validators::serde_json::{json, Value};

use crate::errors::custom_errors::CustomErrors;
use crate::models::users::User;

#[debug_handler]
pub async fn register(
    Extension(pool): Extension<PgPool>,
    Json(credentials): Json<User>,
) -> Result<Json<Value>, CustomErrors> {
    // check if the fields are empty strings
    if credentials.first_name.is_empty()
        || credentials.last_name.is_empty()
        || credentials.email.is_empty()
        || credentials.password.is_empty()
    {
        return Err(CustomErrors::MissingCredential);
    }

    // get the user of the email from the database
    let user = sqlx::query("SELECT email, password FROM users where email = $1")
        .bind(&credentials.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| CustomErrors::InternalServerError)?;

    // if user already exits send user already exits
    if user.is_some() {
        return Err(CustomErrors::UserAlreadyExist);
    }

<<<<<<< HEAD
    let result = sqlx::query(
=======
    sqlx::query(
>>>>>>> origin/main
        "INSERT into users (first_name, last_name, email, password) values ($1, $2, $3, $4)",
    )
    .bind(credentials.first_name)
    .bind(credentials.last_name)
    .bind(credentials.email)
    .bind(credentials.password)
    .execute(&pool)
    .await
    .map_err(|_| CustomErrors::InternalServerError)?;

    Ok(Json(json!("registered successfully")))
}
