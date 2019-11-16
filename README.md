## How to use this framework

### Environment variable

- ADDR: Local address to use
- PORT: Local port
- DB_URI: PostgreSQL database uri for connection
- SESSION_SECRET: Secret used to generate JWT sessions
- SECRET_SALT(optional): External salt for platform password protection

## Directory architecture

Server file are under the src folder.
Thing are gathered in the resource folder under their name.
Each resource folder are build around the following files:

- controller: Manage the routing and the basic control of the resource
- model: Contain the database and api internal structure of the resource
- rule_set: Define all the rules linked to this resource to the authorization_engine. Rules can be used in the entire project

### Presenter

Presentation is manage inside the model structure with the help of the [Serde crate](https://serde.rs/)

## Todo

### Until V0.1

- Proper async db query

### Until usable system

- Unit test
- Mail engine
- Websocket
- Log level and main thread logger
- Session engine improvement (Session renew, update, deletion and expiry)
- Customize error output to make it RESTFull

### Until decent system

- CI/CD
- Validator (Could use serde strict mode?)
- Systemd service
- Setup multi threading
- Rate limit
- oauth support
- Integration testing
- Basic Vue.js SPA frontend
- Some documentations
- Complete implementation for the JWT security (Key rotation, secondary token)
- Output format based on request Accept header
- Create macro helper to load session and database connection

### Current question

- Does the log writing have to be sync once I use multi-threading
- Is it possible in Nickel to register a middleware at the top/bottom position in the stack

### Bonus/Potential features

- Make an example of template use
- Have the front be a Rust WebAssembly app

## External documentation

[Testing JWT security](https://github.com/ticarpi/jwt_tool/wiki)