use std::env;
use bcrypt::{DEFAULT_COST, hash, verify};

fn generate_system_salt() -> String {
    match env::var("SECRET_SALT") {
        Ok(h) => h,
        Err(_) => "".to_string()
    }
}

pub fn hash_password(pwd: &String) -> std::result::Result<std::string::String, bcrypt::BcryptError> {
    let message = format!("{}{}", generate_system_salt(), pwd);

    hash(message, DEFAULT_COST)
}

pub fn check_password(pwd: String, hashed: String) -> std::result::Result<bool, bcrypt::BcryptError> {
    let message = format!("{}{}", generate_system_salt(), pwd);

    verify(message, &hashed)
}
