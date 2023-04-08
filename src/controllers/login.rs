use axum::{Extension, Json};
use validators::serde_json::{json, Value};
use sqlx::PgPool;

use crate::errors::custom_errors::CustomErrors;
use crate::models::users::{RequestUser};



pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(credentials): Json<RequestUser>,
) -> Result<Json<Value>, CustomErrors>{
    // check if the credentials are empty
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(CustomErrors::MissingCredential)
    }

    // check if the user exists
    let user = sqlx::query_as!(RequestUser, r#"SELECT id, email, password FROM users where email=$1"#, credentials.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| CustomErrors::UserDoesNotExist)?;

   
   if let Some(user) = user{
        if user.password != credentials.password {
            Err(CustomErrors::WrongCredential)
        } else{
            Ok(Json(json!({"id": user.id, "email": user.email})))
        }
   }else{
        Err(CustomErrors::UserDoesNotExist)
   }

}