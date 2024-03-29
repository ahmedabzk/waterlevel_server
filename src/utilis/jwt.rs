
use chrono::Duration;
use jsonwebtoken::{Header, EncodingKey, encode, decode, DecodingKey,Validation};
use serde::{Serialize, Deserialize};
use sqlx::{PgPool, Row};


use crate::errors::custom_errors::CustomErrors;
use crate::models::users::TokenUser;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub email: String,
    pub exp: usize,
}

pub async fn create_token(secret: &str, email:String) ->Result<String, CustomErrors> {
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims{exp, email};

    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    let token = encode(&token_header, &claims, &key).map_err(|err|{
        println!("error creating token {:?}", err);
        CustomErrors::TokenCreation
    })?;

    Ok(token)
}

pub async fn verify_token(
    secret: &str, 
    token: &str,
    db: &PgPool,
) -> Result<Option<TokenUser>, CustomErrors> {
    let key = DecodingKey::from_secret(secret.as_bytes());

    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &key, &validation).map_err(|err| match  err.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken
        | jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            CustomErrors::Unauthorized
        }
        _ => {
            println!("failed to verify token {:?}", err);
            CustomErrors::InternalServerError
        }
    })?;

    let user_email = token_data.claims.email;

    let row = sqlx::query("SELECT id, email FROM users WHERE email = $1")
        .bind(user_email)
        .fetch_optional(db)
        .await
        .expect("got none");

    let user = row.map(|r| TokenUser{
        id: r.try_get("id").unwrap(),
        email: r.try_get("email").unwrap(),
       
    });

    Ok(user)

}