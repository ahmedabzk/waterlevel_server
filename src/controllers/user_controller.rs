use axum::{
    Extension,
    Json,
};
// use serde::{Deserialize, Serialize};
// use serde_json;

use sqlx::PgPool;
use validators::serde_json::{json, Value};

//-> You are defining the module vibaya, this means that this module is found in the controller folder, which is wrong

use crate::models::users::User;
use crate::errors::custom_errors::CustomErrors; // -> Fix above error, you should be able to fix this

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
        return Err(CustomErrors::MissingCredential); // -> This doesn't make sense, StatusCode is an enum, return one of the variants
    }

    // get the user of the email from the database

    let user = sqlx::query_as::<_, User>(
        "SELECT email, password FROM users where email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|_err| {
        CustomErrors::InternalServerError; // -> Re-read the docs of the StatusCode variants
    });

    // if user already exits send user already exits
    if let Some(_) = user {
        // -> You can't do this
        return Err(CustomErrors::UserAlreadyExist);
    }

    let result = sqlx::query(
        "INSERT into users (first_name, last_name, email, password) values ($1, %2, $3, $4)",
    )
    .bind(&credentials.first_name)
    .bind(credentials.last_name)
    .bind(credentials.email)
    .bind(credentials.password)
    .execute(&pool)
    .await
    .map_err(|_err| {
        CustomErrors::InternalServerError
    })?; // -> This won't work

    if result.rows_affected() < 1 {
        Err(CustomErrors::InternalServerError)
    } else {
        Ok(Json(json!({"msg": "registered successfuly"})))
    }
}
