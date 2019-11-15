use super::authorization_engine::AuthorizationEngine;

use crate::resource::account::rule_set::set_rules as set_account_rules;

/**
 * The config struct is used to hold the server configuration (See first line of the main)
 * It is loading every engine needed by the application so they can be used by controller 
 */
pub struct Config {
    pub authorization_engine: AuthorizationEngine
}

impl Config {
    pub fn new() -> Config {
        let mut auth_engine = AuthorizationEngine::new();

        set_account_rules(&mut auth_engine);
        Config {
            authorization_engine: auth_engine
        }
    }
}