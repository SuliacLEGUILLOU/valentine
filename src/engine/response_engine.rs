use nickel::{Request, Response, MiddlewareResult, MediaType};

use crate::plugin::Extensible;
use typemap::Key;

use crate::engine::utils::test_empty_vec;
use crate::resource::account::model::Model as Account;

#[derive(Serialize)]
pub struct BodyResponse {
    pub status: String,
    #[serde(skip_serializing)]
    pub lock: bool,
    #[serde(skip_serializing_if="test_empty_vec")]
    pub account: Vec<Account>,
}
impl Key for BodyResponse { type Value = BodyResponse; }

/**
 * Request initializer
 * TODO: You have to modify the creation of the body to match the structure
 */
fn init_request<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let body = BodyResponse {
        status: String::from("OK"),
        lock: false,
        account: vec![],
    };

    res.extensions_mut().insert::<BodyResponse>(body);
    res.next_middleware()
}

// Attache the response middleware to the server
pub fn attache_init(server: &mut nickel::Nickel) {
    server.utilize(init_request);
}

/**
 * Get the BodyResponse from res and send it back in JSON
 * Parsing helper to avoid a move of res in finalize_request
 * TODO: See how to give it a type
 */
fn get_json(res: &Response) -> String {
    let body = res.extensions().get::<BodyResponse>().unwrap();
    
    serde_json::to_string(body).unwrap()
}

/**
 * Finalize the request by putting it into the JSON format
 * TODO: This can easily be expanded to support more output format based on req header
 * TODO: Make this more generic so it's easier to use
 * TODO: The code of the response body should be dealt with in the engine and not at controller level
 */
fn finalize_request<'mw>(_req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    res.set(MediaType::Json);

    let body = get_json(&res);
    res.send(body)
}

// Attache the response middleware to the server
pub fn attache_final(server: &mut nickel::Nickel) {
    server.utilize(finalize_request);
}