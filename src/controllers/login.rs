use axum::extract::State;
use axum::{Json};
use validators::serde_json::{json, Value};
use sqlx::PgPool;


use crate::errors::custom_errors::CustomErrors;
use crate::models::users::{RequestUser, ResponseUser};
use crate::utilis::password::verify_password;
use crate::utilis::jwt::create_token;
use crate::utilis::token_wrapper::TokenWrapper;




pub async fn login(
    State(db): State<PgPool>,
    State(token_secret): State<TokenWrapper>,
    Json(credentials): Json<RequestUser>,
) -> Result<Json<Value>, CustomErrors>{
    // check if the credentials are empty
    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(CustomErrors::MissingCredential)
    }

    // check if the user exists
    let user = sqlx::query_as!(RequestUser, r#"SELECT id, email, password FROM users where email=$1"#, credentials.email)
        .fetch_optional(&db)
        .await
        .map_err(|_| CustomErrors::UserDoesNotExist)?;

   
   if let Some(user) = user{
        let password_verification = verify_password(&credentials.password, &user.password).await?;
         if !password_verification{
            Err(CustomErrors::WrongCredential)
        } else{
            let token = create_token(&token_secret.0, credentials.email).await?;
            let response = ResponseUser{
                id: user.id,
                email: user.email,
                token,
            };

            Ok(Json(json!(response)))
        }
   }else{
        Err(CustomErrors::UserDoesNotExist)
   }

}