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
- presenter: Contain the api external structure of the resource

## Todo

### Until V0.1

- Proper async db query

### Until usable system

- Unit test
- Mail engine
- Websocket
- Log level and main thread logger
- Improve the response engine format to make it more generic
- Session engine improvement (Session update, deletion and expiry)
- Presenter system for resource
- Authorization engine

### Until decent system

- CI/CD
- Validator
- Systemd service
- Setup multi threading
- Rate limit
- oauth support
- Integration testing
- Basic Vue.js SPA frontend
- Some documentations
- Customize error output to make it RESTFull
- Complete implementation for the JWT security (Key rotation, secondary token)

### Bonus/Potential features

- Make an example of template use
- Have the front be a Rust WebAssembly app
- Multiple output support
