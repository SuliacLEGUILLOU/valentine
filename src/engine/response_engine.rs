use nickel::hyper::header::{ContentEncoding, Encoding};
use nickel::{Nickel, MediaType, MiddlewareResult, Request, Response};

use crate::plugin::Extensible;
use typemap::Key;

use crate::engine::utils::test_empty_vec;
use crate::resource::account::model::Model as Account;

use libflate::gzip::Encoder;
use std::io;

use super::config_engine::Config;

/**
 * This structure represents the basic body that's going to be use by the api
 * TODO: See if this could be injected into the engine to make it more extendable
 */
#[derive(Serialize)]
pub struct BodyResponse {
    pub status: String,
    #[serde(skip_serializing)]
    pub lock: bool,
    #[serde(skip_serializing_if = "test_empty_vec")]
    pub account: Vec<Account>,
}

// Implement the key trait so the struct can be passed in the middleware stack
impl Key for BodyResponse {
    type Value = BodyResponse;
}

/**
 * Request initializer
 * TODO: You have to modify the creation of the body to match the structure
 */
fn init_request<'mw>(_req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = BodyResponse {
        status: String::from("OK"),
        lock: false,
        account: vec![],
    };

    res.extensions_mut().insert::<BodyResponse>(body);
    res.next_middleware()
}

// Attache the response init middleware to the server
pub fn attache_init(server: &mut Nickel<Config>) {
    server.utilize(init_request);
}

/**
 * Get the BodyResponse from res and send it back in JSON
 * Parsing helper to avoid a move of res in finalize_request
 * TODO: See how to give it a type
 */
fn get_json(res: &Response<Config>) -> String {
    let body = res.extensions().get::<BodyResponse>().unwrap();

    serde_json::to_string(body).unwrap()
}

/**
 * Finalize the request by putting it into the JSON format and compressing it
 * TODO: This can easily be expanded to support more output format based on req header
 */
fn finalize_request<'mw>(_req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    res.set(MediaType::Json)
        .set(ContentEncoding(vec![Encoding::Gzip]));

    io::copy(&mut get_json(&res).as_bytes(), &mut encoder).unwrap();
    res.send(encoder.finish().into_result().unwrap())
}

// Attache the response final middleware to the server
pub fn attache_final(server: &mut Nickel<Config>) {
    server.utilize(finalize_request);
}
