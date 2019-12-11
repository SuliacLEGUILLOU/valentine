use nickel::status::StatusCode;
use nickel::{HttpRouter, JsonBody, MiddlewareResult, Request, Response};

use crate::plugin::Extensible;

use nickel_postgres::PostgresRequestExtensions;

use super::model::Model as Account;

use crate::engine::response_engine::BodyResponse;
use crate::engine::session_engine::Session;

use crate::engine::config_engine::Config;

pub fn add_route(router: &mut nickel::Router<Config>) {
    router.get("/account", get_all);
    router.get("/account/:id", get);
    router.post("/account", post);
    router.patch("/account/:id", patch);
    router.delete("/account/:id", delete);
}

fn get_all<'mw>(req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = res.extensions_mut().get_mut::<BodyResponse>().unwrap();
    let conn = try_with!(res, req.pg_conn());

    body.account.append(&mut Account::get_all(conn));
    res.next_middleware()
}

fn get<'mw>(req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = res.extensions_mut().get_mut::<BodyResponse>().unwrap();
    let conn = try_with!(res, req.pg_conn());
    let id = req.param("id").unwrap();

    body.account.append(&mut Account::get_by_id(conn, id));
    res.next_middleware()
}

fn post<'mw>(req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = res.extensions_mut().get_mut::<BodyResponse>().unwrap();
    let conn = try_with!(res, req.pg_conn());
    let mut account = try_with!(res, {
        req.json_as::<Account>()
            .map_err(|e| (StatusCode::BadRequest, e))
    });

    account.insert(conn);
    body.account.push(account);
    res.next_middleware()
}

fn patch<'mw>(req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = res.extensions_mut().get_mut::<BodyResponse>().unwrap();
    let conn = try_with!(res, req.pg_conn());
    let mut account = try_with!(res, {
        req.json_as::<Account>()
            .map_err(|e| (StatusCode::BadRequest, e))
    });
    let session = match req.extensions().get::<Session>() {
        Some(s) => s,
        None => return res.error(StatusCode::Forbidden, "Forbidden")
    };
    let config = req.server_data();

    if config.authorization_engine.check_rule("IS_ACCOUNT", req) == false {
        return res.error(StatusCode::Forbidden, "Wrong user");
    }

    account.id = session.id.clone();
    account.patch(conn);
    body.account.push(account);
    res.next_middleware()
}

fn delete<'mw>(req: &mut Request<Config>, mut res: Response<'mw, Config>) -> MiddlewareResult<'mw, Config> {
    let body = res.extensions_mut().get_mut::<BodyResponse>().unwrap();
    let config = req.server_data();
    let conn = try_with!(res, req.pg_conn());
    let account = Account {
        id: req.param("id").unwrap().to_string(),
        name: "".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };

    if config.authorization_engine.check_rule("IS_ACCOUNT", req) == false {
        return res.error(StatusCode::Forbidden, "Wrong user");
    }

    account.delete(conn);
    body.account.push(account);
    res.next_middleware()
}
