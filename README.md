
## Directory architecture

Server file are under the src folder.
Thing are gathered in the resource folder under their name.
Each resource folder are build around the following files:

- controller: Manage the routing and the basic control of the resource
- model: Contain the database and api internal structure of the resource
- presenter: Contain the api external structure of the resource

## Todo

### Until V0.1

- Session engine (JWT) and authorization
- Some response_engine in charge of formatting the result of the request (Require a good dig at the middleware)
- Proper async db query

### Until usable system

- Unit test
- Mail engine
- Improve log
- Websocket

### Until decent system

- CI/CD
- Validator
- Systemd service
- Setup multi threading
- Rate limit
- oauth support
