# Building a microservice in rust

This project is a companion to my talk at ConFoo about building a rust project.

The actual code is in different branches.

## Step 1

This step adds the dependencies like `tokio`, `anyhow` and `tracing_subscriber`.
This sets the project up for non-blocking IO and adds logging and error handling
libraries.

This also configures the binary to log if `RUST_LOG=debug` environment variable
is set.

```
RUST_LOG=debug cargo run
```

## Step 2

This steps refactors the application into a library and a binary. This
allows multiple binary endpoints that may use the same underlying crate for
things like the SQS worker or the HTTP handler.

## Step 3

This step adds axum and sets up a hello world endpoint.

* Setup and start server - `./src/server.rs`
* Router - `./src/router.rs`
* Error - `./src/error.rs`

Run `git checkout step-04` to get to the next step.


* [x] Bootstrap Rust project
* [x] Add tokio and anyhow and run as an async binary
* [x] Library and binary structure and setup tracing
* [x] Add axum and setup a hello world endpoint
* [ ] Explore different extractors for Path, Body and Query Params
* [ ] Add database support with sqlx and postgres
* [ ] Authentication based on a JWT token
