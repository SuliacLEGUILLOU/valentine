use nickel::{HttpRouter, Request, Response, MiddlewareResult, JsonBody};
use nickel::status::StatusCode;

use crate::plugin::Extensible;
use typemap::Key;

use nickel_postgres::PostgresRequestExtensions;

use super::model::Model as Account;

pub fn add_route(router: &mut nickel::Router) {
    router.get("/account", get_all);
    router.get("/account/:id", get);
    router.post("/account", post);
    router.patch("/account/:id", patch);
    router.delete("/account/:id", delete);
}

#[derive(Serialize, Deserialize)]
pub struct RawResponse {
    status: String,
    account: Vec<Account>,
}

fn get_all<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let body = RawResponse {
        status: String::from("OK"),
        account: Account::get_all(conn)
    };

    res.extensions_mut().insert::<RawResponse>(body);
    res.next_middleware()
}

impl Key for RawResponse { type Value = RawResponse; }

fn get<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let id = req.param("id").unwrap();

    let body = RawResponse {
        status: String::from("OK"),
        account: Account::get_by_id(conn, id)
    };

    res.extensions_mut().insert::<RawResponse>(body);
    res.next_middleware()
}

fn post<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let mut account = try_with!(res, {
        req.json_as::<Account>().map_err(|e| (StatusCode::BadRequest, e))
    });

    account.insert(conn);

    let body = RawResponse {
        status: String::from("CREATED"),
        account: vec![account]
    };
    res.extensions_mut().insert::<RawResponse>(body);
    res.next_middleware()
}

fn patch<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let mut account = try_with!(res, {
        req.json_as::<Account>().map_err(|e| (StatusCode::BadRequest, e))
    });

    account.id = req.param("id").unwrap().to_string();
    account.patch(conn);

    let body = RawResponse {
        status: String::from("ACCEPTED"),
        account: vec![account]
    };
    res.extensions_mut().insert::<RawResponse>(body);
    res.next_middleware()
}

fn delete<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let account = Account {
        id: req.param("id").unwrap().to_string(),
        name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };

    account.delete(conn);

    let body = RawResponse {
        status: String::from("ACCEPTED"),
        account: vec![account]
    };
    res.extensions_mut().insert::<RawResponse>(body);
    res.next_middleware()
}