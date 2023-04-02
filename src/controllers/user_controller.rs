use serde_json;
use serde::{
    Serialize,
    Deserialize,
};
use axum::{
    StatusCode,
    Extension, Json,
};

use sqlx::PgPool;
use validators::serde_json::json;

mod models;

use models::users::User;


pub async fn register(
Json(credentials): Json<users::User>,
Extension(pool): Extension<PgPool>
) -> Result<Json<Value>, StatusCode>{
    // check if the fields are empty strings
    if credentials.first_name.is_empty() 
    || credentials.last_name.is_empty() 
    || credentials.email.is_empty() 
    || credentials.password.is_empty(){
        return Err(StatusCode)
    }

    // get the user of the email from the database

    let user = sqlx::query_as::<_, models::users::User>(
        "SELECT email, password FROM users where email = $1",
    )
    .bind(&credentials.email)
    .fetch_optional(&pool)
    .await
    .map_err(|err| {
        dbg!(err);
        StatusCode::InternalServerError;
    });

    // if user already exits send user already exits
    if let Some(_) = user {
        return Err(StatusCode::BadRequest)
    }

    let result = sqlx::query("INSERT into users (first_name, last_name, email, password) values ($1, %2, $3, $4)")
        .bind(&credentials.first_name)
        .bind(credentials.last_name)
        .bind(credentials.email)
        .bind(credentials.password)
        .execute(&pool)
        .await
    .map_err(|err|{
        dbg!(err);
        return Err(StatusCode::InternalServerError)
    })?;

    if result.rows_affected() < 1{
        Err(StatusCode::InternalServerError)
    }else {
        Ok(Json(json!({"msg": "registered successfuly"})))
    }
}