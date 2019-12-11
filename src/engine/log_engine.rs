use chrono::prelude::*;
use nickel::{MiddlewareResult, Request, Response};
use std::sync::mpsc::{self, Sender, Receiver};

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

pub struct Logger {
    tx: Sender<String>,
}

impl Logger {
    pub fn new() -> Logger {
        let (tx, rx) = mpsc::channel();
        let logger = Logger { tx: tx };

        let f = Logger::launch(rx);
        return logger;
    }

    async fn launch(rx: Receiver<String>) {
        for received in rx {
            println!("{}", received);
        }
    }

    pub fn debug(&self, message: String) {
        self.tx.send(format!(
            "[DEBUG] {} {}",
            Utc::now().format("%b %e %T"),
            message
        )).unwrap();
    }
}
