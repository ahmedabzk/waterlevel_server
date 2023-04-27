use axum::{
    extract::State, 
    http::{HeaderMap, Request}, 
    middleware::Next, 
    response::Response,
};

use sqlx::postgres::PgPool;

use crate::{utilis::token_wrapper::TokenWrapper};
use crate::utilis::jwt::verify_token;
use crate::errors::custom_errors::CustomErrors;




pub async fn require_auth<T>(
    State(token_secret): State<TokenWrapper>,
    State(db): State<PgPool>,
    header: HeaderMap,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, CustomErrors>
where
T: Send + 'static
{
    
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
        .map_err(|err| dbg!(err))?;
   
   if user.is_some(){
        request.extensions_mut().insert(user);
   }else{
        println!("user is none");
        return  Err(CustomErrors::Unauthorized)
   }
   
   Ok(next.run(request).await)
     

}

