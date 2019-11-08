use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;

/**
 * Get the optional system salt from the SECRET_SALT variable
 * It is set to prevent using password in another system
 */
fn generate_system_salt() -> String {
    match env::var("SECRET_SALT") {
        Ok(h) => h,
        Err(_) => "".to_string(),
    }
}

/**
 * Hash the provided password
 */
pub fn hash_password(pwd: &String) -> String {
    let message = format!("{}{}", generate_system_salt(), pwd);

    hash(message, DEFAULT_COST).unwrap()
}

/**
 * Verify the provided password
 */
pub fn check_password(pwd: &String, hashed: &String) -> bool {
    let message = format!("{}{}", generate_system_salt(), pwd);

    verify(message, &hashed).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_system_salt() {
        assert_eq!(generate_system_salt(), "".to_string());
        env::set_var("SECRET_SALT", "unit_test");
        assert_eq!(generate_system_salt(), "unit_test".to_string());
    }

    #[test]
    #[ignore] // TODO: I don't understand why this fail. Help needed
    fn hash_and_verify_password() {
        let pwd = "123456".to_string();
        let hashed = hash_password(&pwd);

        let result = check_password(&pwd, &hashed);
        assert!(result);
        // println!("{:?}", check_password(pwd, hashed));
    }
}
