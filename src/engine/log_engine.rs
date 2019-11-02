use nickel::{Request, Response, MiddlewareResult};

// TODO: Add more information in each request
fn log_request<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("request: {} {}", req.origin.method, req.origin.uri);
    res.next_middleware()
}

pub fn attache(server: &mut nickel::Nickel) {
    server.utilize(log_request);
}