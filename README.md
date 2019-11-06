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
- model: Contain the database and api internal structure of the resource.

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
- Session engine improvement (Session update, deletion and expiry)
- Authorization engine

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
- Customize error output to make it RESTFull
- Complete implementation for the JWT security (Key rotation, secondary token)
- Output format based on request

### Current question

- Is it possible in Nickel to register a middleware at the top/bottom position in the stack

### Bonus/Potential features

- Make an example of template use
- Have the front be a Rust WebAssembly app
