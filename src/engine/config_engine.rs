use super::authorization_engine::AuthorizationEngine;

use crate::resource::account::rule_set::set_rules as set_account_rules;

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