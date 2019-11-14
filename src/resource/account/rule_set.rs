use nickel::Request;

use crate::plugin::Extensible;

use crate::engine::authorization_engine::AuthorizationEngine;
use crate::engine::config_engine::Config;
use crate::engine::session_engine::Session;

/**
 * Set all the defined rule in the provided authorization engine
 */
pub fn set_rules(engine: &mut AuthorizationEngine){
    engine.set("IS_ACCOUNT", is_account);
    engine.set("IS_ACCOUNT_STRICT", is_account_strict);
}

/**
 * Return true if the session user id is matching the param account_id
 * If no account_id is provided, try with the id field
 */
fn is_account(req: &Request<Config>) -> bool {
    let session = req.extensions().get::<Session>().unwrap();
    
    match req.param("account_id") {
        Some(id) => return id.to_string() == session.id,
        None => {}
    }

    match req.param("id") {
        Some(id) => return id.to_string() == session.id,
        None => return false
    }
}

/**
 * Return true if the session user id is matching the param user_id
 */
fn is_account_strict(req: &Request<Config>) -> bool {
    let session = req.extensions().get::<Session>().unwrap();
    
    match req.param("account_id") {
        Some(id) => id.to_string() == session.id,
        None => false
    }
}