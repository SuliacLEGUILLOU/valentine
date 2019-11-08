use chrono::prelude::*;
use nickel::{MiddlewareResult, Request, Response};

// Logger middleware function: Once attached to the server this log every request details
// TODO: Add more information in each request
fn log_request<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!(
        "[DEBUG] {} {} {}",
        Utc::now().format("%b %e %T"),
        req.origin.method,
        req.origin.uri
    );
    res.next_middleware()
}

// Attache the logger middleware to the server
pub fn attache(server: &mut nickel::Nickel) {
    server.utilize(log_request);
}
