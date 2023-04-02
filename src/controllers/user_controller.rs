use axum::{
    Extension,
    Json,
    StatusCode, // -> Wrong import, this enum should be found in axum::http::StatusCode
};
use serde::{Deserialize, Serialize};
use serde_json;

use sqlx::PgPool;
use validators::serde_json::json;

mod models; //-> You are defining the module vibaya, this means that this module is found in the controller folder, which is wrong

use models::users::User; // -> Fix above error, you should be able to fix this

pub async fn register(
    Json(credentials): Json<users::User>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, StatusCode> {
    // check if the fields are empty strings
    if credentials.first_name.is_empty()
        || credentials.last_name.is_empty()
        || credentials.email.is_empty()
        || credentials.password.is_empty()
    {
        return Err(StatusCode); // -> This doesn't make sense, StatusCode is an enum, return one of the variants
    }

    // get the user of the email from the database

    let user = sqlx::query_as::<_, models::users::User>(
        "SELECT email, password FROM users where email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        // -> Bad error handling
        dbg!(err);
        StatusCode::InternalServerError; // -> Re-read the docs of the StatusCode variants
    });

    // if user already exits send user already exits
    if let Some(_) = user {
        // -> You can't do this
        return Err(StatusCode::BadRequest);
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
    .map_err(|err| {
        dbg!(err);
        return Err(StatusCode::InternalServerError);
    })?; // -> This won't work

    if result.rows_affected() < 1 {
        Err(StatusCode::InternalServerError)
    } else {
        Ok(Json(json!({"msg": "registered successfuly"})))
    }
}
