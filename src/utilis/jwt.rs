use chrono::Duration;
use jsonwebtoken::{Header, EncodingKey, encode, decode, DecodingKey,Validation};
use serde::{Serialize, Deserialize};

use crate::errors::custom_errors::CustomErrors;

#[derive(Serialize, Deserialize, Debug)]
struct Claims{
    username: String,
    exp: usize,
}

pub async fn create_token(secret: &str, username: String) ->Result<String, CustomErrors> {
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims{exp, username};

    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|err|{
        println!("error creating token {:?}", err);
        CustomErrors::TokenCreation
    })

    
}

pub async fn verify_token(secret: &str, token: &str) -> Result<bool, CustomErrors> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation).map_err(|err| match  err.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken
        | jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            CustomErrors::Unauthorized
        }
        _ => {
            println!("failed to verify token {:?}", err);
            CustomErrors::InternalServerError
        }
    })
    .map(|_claim| true)
}