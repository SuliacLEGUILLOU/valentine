use std::collections::HashMap;
use nickel::Request;

use super::config_engine::Config;

/**
 * Set Rule type
 */
type Rule = fn(req: &Request<Config>) -> bool;

/**
 * Structure used to hold the authorizations settings
 * TODO: Should this have a conflict resolution strategy? (default panic, replace, ignore?)
 */
pub struct AuthorizationEngine {
    auth_map: HashMap<String, Rule>,
}

impl AuthorizationEngine {
    /**
     * Return a new Authorization engine
     */
    pub fn new() -> AuthorizationEngine {
        AuthorizationEngine {
            auth_map: HashMap::new()
        }
    }

    /**
     * Set a new rule in the engine
     * Panic if a rule with the same name already exist
     */
    pub fn set(&mut self, name: &str, func: Rule) {
        if self.auth_map.contains_key(&name.to_string()) {
            panic!("RULE CONFLICT: you are trying to set the same rule twice ({})", name);
        }
        self.auth_map.insert(name.to_string(), func);
    }

    /**
     * Run a rule check and return its result
     *?TODO: Should an undefined rule panic or always return false?
     */
    pub fn check_rule(&self, name: &str, req: &Request<Config>) -> bool {
        match self.auth_map.get(&name.to_string()) {
            Some(func) => func(req),
            None => panic!("RULE {} IS NOT SET", name)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth(_req: &Request<Config>) -> bool {
        true
    }

    fn no_auth(_req: &Request<Config>) -> bool {
        false
    }

    // #[test]  // TODO: This test works without the Request param, ask nickel dev about how to mock them
    // fn test_new() {
    //     let mut x1 = AuthorizationEngine::new();
    //     let r: Request<'_, '_, _> = Request::new();

    //     x1.set("AUTH", auth);
    //     x1.set("NO_AUTH", no_auth);
    //     x1.set("LAMBDA_AUTH", {|req|
    //         true
    //     });

    //     assert!(x1.check_rule("AUTH", &r));
    //     assert!(x1.check_rule("LAMBDA_AUTH", &r));
    //     assert!(!x1.check_rule("NO_AUTH", &r));
    //     assert!(!x1.check_rule("NO_RULE", &r));
    // }

    #[test]
    #[should_panic]
    fn test_conflict() {
        let mut x1 = AuthorizationEngine::new();

        x1.set("HELLO", auth);
        x1.set("HELLO", no_auth);
    }
}