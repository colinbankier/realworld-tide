# ![RealWorld Example App](logo.png)

> ### Rust/Tide codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.

[Demo](https://github.com/gothinkster/realworld)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

# Overview

This codebase was created to demonstrate a fully fledged backend application built with **Rust** and [**`tide`**](https://github.com/colinbankier/realworld-tide) including CRUD operations, authentication, routing, pagination, and more.

This project attempts to achieve a clear separation between web, domain and persistence logic -
loosely along the lines of the [ports and adapters architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)).  
These three concerns live in separate crates - `web`, `domain` and `db`.  
`tide` is used in the `web` crate while `diesel` is the main character of the `db` crate.  
The fourth crate, `application`, glues the other three together and provides the runnable binary.

Each sub-crate has its own set of tests, with integration tests taking place in the `web` crate.

You can also exercise the application using Realworld's Postman collection: [here](https://github.com/gothinkster/realworld/tree/master/api).

For more information on how this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.

## Other frameworks

If you want to get a taste of what it feels like to build a Realworld's implementation using another
Rust's web framework, you can reuse the `domain` and `db` sub-crate. 

You just need to provide an alternative implementation of the `web` crate leveraging your framework of choice.

# Getting started

## Setup

### Prerequisites

- Rust 1.39 (see [here](https://www.rust-lang.org/tools/install) for instructions)
- Docker (see [here](https://docs.docker.com/install/) for instructions)
- Postgres (see [here](https://www.postgresql.org/download/) for instructions)

### Setup steps
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
# 5433 is the port specified in configuration/development.yml, the default choice
POSTGRES_PORT=5433 ./scripts/init_db.sh
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
APIURL=http://localhost:5000/api ./run-api-tests.sh
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
