use axum::extract::FromRef;
use sqlx::PgPool;

use crate::utilis::token_wrapper::TokenWrapper;


#[derive(Clone, FromRef)]
pub struct AppState{
    pub db: PgPool,
    pub jwt_secret: TokenWrapper,
}