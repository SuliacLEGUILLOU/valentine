
## Directory architecture

Server file are under the src folder.
Thing are gathered in the resource folder under their name.
Each resource folder are build around the following files:

- controller: Manage the routing and the basic control of the resource
- model: Contain the database and api internal structure of the resource
- presenter: Contain the api external structure of the resource

## Todo

### Until usable system

- Session engine (JWT) and authorization
- Unit test
- Proper async db query
- Mail engine
- Improve log
- Websocket

### Until decent system

- Some response_engine in charge of formatting the result of the request (Require a good dig at the middleware)
- CI/CD
- Validator
- Systemd service
- Setup multi threading
- Rate limit
- oauth support
