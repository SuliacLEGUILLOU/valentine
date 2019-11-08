use crypto::sha2::Sha512;
use jwt::{Header, Registered, Token};
use nickel::hyper::header::{self, Authorization, Bearer};
use nickel::status::StatusCode;
use nickel::{HttpRouter, JsonBody, MediaType, Middleware, MiddlewareResult, Request, Response};
use nickel_postgres::PostgresRequestExtensions;

use crate::plugin::Extensible;
use typemap::Key;

use crate::resource::account::model::Model as Account;

/**
 * Response structure for the post /session endpoint
 */
#[derive(Serialize, Deserialize, Debug)]
struct ResBody {
    code: String,
    token: String,
}

/**
 * Body structure for the post /session endpoint
 */
#[derive(Serialize, Deserialize, Debug)]
struct ReqBody {
    email: String,
    password: String,
}

/**
 * Session structure
 * This is inserted into the Request structure.
 */
// Allow to store userId in the session's map
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    pub id: String,
}
impl Key for Session {
    type Value = Session;
}

// TODO Find a way to make those one structure (Need to see how to have multiple mw invoke on struct)
struct SessionManager {
    secret: String,
}
struct SessionCreation {
    secret: String,
}

/**
 * Object middleware managing the session integrity
 * TODO: Need ways to expire session by expiry and black list
 */
impl<D> Middleware<D> for SessionManager {
    fn invoke<'mw, 'conn>(
        &self,
        req: &mut Request<'mw, 'conn, D>,
        res: Response<'mw, D>,
    ) -> MiddlewareResult<'mw, D> {
        let auth_header = match req.origin.headers.get::<Authorization<Bearer>>() {
            Some(header) => header,
            None => return res.next_middleware(),
        };

        // TODO: Quick search could not get me the no-deprecated version
        let jwt = header::HeaderFormatter(auth_header).to_string();
        let jwt_slice = &jwt[7..];
        let token = Token::<Header, Registered>::parse(jwt_slice).unwrap();

        if token.verify(&self.secret.as_bytes(), Sha512::new()) {
            req.extensions_mut().insert::<Session>(Session {
                id: token.claims.sub.unwrap(),
            });
            res.next_middleware()
        } else {
            res.error(StatusCode::Forbidden, "Access denied")
        }
    }
}

/**
 * Object middleware managing the session creation
 */
impl<D> Middleware<D> for SessionCreation {
    fn invoke<'mw, 'conn>(
        &self,
        req: &mut Request<'mw, 'conn, D>,
        mut res: Response<'mw, D>,
    ) -> MiddlewareResult<'mw, D> {
        let conn = try_with!(res, req.pg_conn());
        let req_body = try_with!(res, {
            req.json_as::<ReqBody>()
                .map_err(|e| (StatusCode::BadRequest, e))
        });
        let account = Account::get_for_login(conn, req_body.email, req_body.password);

        let header: Header = Default::default();
        let claims = Registered {
            sub: Some(account.id.into()),
            ..Default::default()
        };

        let res_body = ResBody {
            code: "OK".to_string(),
            token: Token::new(header, claims)
                .signed(self.secret.as_bytes(), Sha512::new())
                .unwrap(),
        };

        res.set(MediaType::Json);
        res.send(serde_json::to_string(&res_body).unwrap())
    }
}

// Attach the session check middleware to a server
pub fn attach(server: &mut nickel::Nickel, session_secret: &String) {
    server.utilize(SessionManager {
        secret: session_secret.clone(),
    });
}

// Register route for the session management route
pub fn register_session_route(router: &mut nickel::Router, session_secret: &String) {
    router.post(
        "/session",
        SessionCreation {
            secret: session_secret.clone(),
        },
    );
}
