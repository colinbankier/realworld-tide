# ![RealWorld Example App](logo.png)

> ### Rust/Tide codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.


### [Demo](https://github.com/gothinkster/realworld)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

### WIP - this repo is as yet incomplete and still being implemented

This codebase was created to demonstrate a fully fledged fullstack application built with **Rust/Tide** including CRUD operations, authentication, routing, pagination, and more.

This project attempts to achieve the following:
 - Separate domain logic from web logic. The `conduit` module contains domain logic and the `web` module has logic for dealing with http stuff and json request/response formats.
 - Async queries with diesel. Diesel doesn't directly support async, but we can still build an async application around it using `tokio_threadpool::blocking`. The `db` module provides a `Repo` abstraction to encapsulate this.
 - Parallel database tests. Tests use isolated test transactions so database tests can be run in parallel.
 - HTTP level integration tests for the web layer. The `test_helpers` module provides a `TestServer` to easily simulate http requests for tests.

The app will evolve as I experiment with nice ways to structure things. It's very minimal so far, but I intend to grow it to be a good reference for implementing an app in Tide.

The integration tests in the web layer cover the features implemented so far, which does not yet cover all cases required for the realworld spec.

The realworld Postman collection can be used to drive the next set of features to be added (see [https://github.com/gothinkster/realworld/tree/master/api]). See steps below for how to run these.


We've gone to great lengths to adhere to the **Rust/Tide** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.


# How it works

> Describe the general architecture of your app here

# Getting started

## Setup

### Prerequisites

- Rust (see [here](https://www.rust-lang.org/tools/install) for instructions)
- Docker (see [here](https://docs.docker.com/install/) for instructions)
- Postgres (see [here](https://www.postgresql.org/download/) for instructions)

### Setup steps
- Set the channel for this project to nightly:
```bash
# Execute in the top-level folder of the project
rustup override set nightly
```
- Install the `diesel` CLI:
```bash
cargo install diesel_cli --no-default-features --features postgres
```
- Launch a local Postgres instance and run SQL migrations:
```bash
./scripts/init_db.sh
```

You are ready to go!

## Run tests
Run tests, including DB integration tests

```bash
# This will launch a Postgres instance in a docker container.
# You can customise its behaviour using env variables:
# - database name, using POSTGRES_DB
# - user, using POSTGRES_USER
# - password, using POSTGRES_PASSWORD
# - port, using POSTGRES_PORT
# 5434 is the port specified in configuration/test.yml, the test configuration file
POSTGRES_PORT=5434 ./scripts/init_db.sh
# Execute the tests
./scripts/run_tests.sh
```

## Run app and realworld test suite
Run the app
```bash
# This will launch a Postgres instance in a docker container.
# You can customise its behaviour using env variables:
# - database name, using POSTGRES_DB
# - user, using POSTGRES_USER
# - password, using POSTGRES_PASSWORD
# - port, using POSTGRES_PORT
# 5435 is the port specified in configuration/development.yml, the default choice
POSTGRES_PORT=5435 ./scripts/init_db.sh
# Launch the application!
cargo run
```
You can pass the `--release` flag to squeeze in the last drop of performance.

By default, we look for Postgres on a different port when executing tests - hence you can run the test suite
and interact with the application locally without suffering any interference.

If you want to run the "realworld" Postman tests, just execute
```bash
git clone https://github.com/gothinkster/realworld
cd realworld/api
APIURL=http://localhost:8181/api ./run-api-tests.sh
```

## Configuration

All configuration files are in the `configuration` folder.

The default settings are stored in `configuration/base.yml`.

Environment-specific configuration files can be used to override or supply additional values on top the
default settings (see `development.yml` or `test.yml`).
In a production environment, you could introduce a `production.yml` to store production-specific configuration values.

Configuration files can also be overriden using environment variables, prefixed with `APP`: 
the value of `APP_APPLICATION_PORT` will have higher priority then `application.port` in `base.yml` or `development.yml`.

All configurable parameters are listed in `configuration.rs`.