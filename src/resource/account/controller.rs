use nickel::{HttpRouter, Request, Response, MiddlewareResult, JsonBody, MediaType};
use nickel::status::StatusCode;
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
struct RawResponse {
    status: String,
    account: Vec<Account>,
}

fn get_all<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let body = RawResponse {
        status: String::from("OK"),
        account: Account::get_all(conn)
    };

    res.set(MediaType::Json);
    res.send(serde_json::to_string(&body).unwrap())
}

fn get<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let conn = try_with!(res, req.pg_conn());
    let id = req.param("id").unwrap();
    
    let mut body = RawResponse {
        status: String::from("OK"),
        account: Account::get_by_id(conn, id)
    };

    if body.account.len() == 0 {
        body.status = String::from("NOT_FOUND");
    }
    res.set(MediaType::Json);
    res.send(serde_json::to_string(&body).unwrap())
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
    res.set(MediaType::Json);
    res.send(serde_json::to_string(&body).unwrap())
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
    res.set(MediaType::Json);
    res.send(serde_json::to_string(&body).unwrap())
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
    res.set(MediaType::Json);
    res.send(serde_json::to_string(&body).unwrap())
}