use nickel::{Request, Response, MiddlewareResult, MediaType};

use crate::plugin::Extensible;

use crate::resource::account::controller::RawResponse;

/**
 * Get the RawResponse from res and send it back in JSON
 * Parsing helper to avoid a move of res in finalize_request
 * TODO: See how to give it a type
 */
fn get_json(res: &Response) -> String {
    serde_json::to_string(res.extensions().get::<RawResponse>().unwrap()).unwrap()
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
pub fn attache(server: &mut nickel::Nickel) {
    server.utilize(finalize_request);
}