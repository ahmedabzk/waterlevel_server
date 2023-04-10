use crate::errors::custom_errors::CustomErrors;
use bcrypt::{hash, verify};


// hash password function
pub async fn hash_password(password:&str) -> Result<String, CustomErrors>{
    hash(password,9).map_err(|err|{
        println!("failed to hash password {:?}", err);
        CustomErrors::InternalServerError
    })
}

// verify password function
pub async fn verify_password(provided_password: &str, password:&str) -> Result<bool, CustomErrors> {
    verify(provided_password, password).map_err(|err|{
        println!("The given password is not equal to the password stored in the database {:?}", err);
        CustomErrors::WrongCredential
    })
}