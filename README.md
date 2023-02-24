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

## Step 4

This step adds `serde` and `serde_json` for serializing structs. We need
this for reading the request body and returning JSON.

* Path Parameters - `./src/router/path_handler.rs`
* Query Parameters - `./src/router/query_handler.rs`
* JSON body handler - `./src/router/json_handler.rs`

## Step 5

This steps adds database handling to our service. Here we are using
postgresql with sqlx. We are also going to add chrono for our date 
types and async_trait to build a repository trait and an implementation
for it using the concrete database. This will help us test the router
with just a mock implementation of the trait while the DAO tests can
hit the actual database.

We'll need an environment variable called `DATABASE_URL` for cargo to
build and run type checked SQL. You can opt out of it but I prefer it.

I've set my `DATABASE_URL` to `postgres:///testdb`

DAO - `./src/dao/mod.rs`
Tasks - `./src/dao/tasks.rs`

Run `git checkout step-06` to get to the next step.


* [x] Bootstrap Rust project
* [x] Add tokio and anyhow and run as an async binary
* [x] Library and binary structure and setup tracing
* [x] Add axum and setup a hello world endpoint
* [x] Explore different extractors for Path, Body and Query Params
* [ ] Add database support with sqlx and postgres
* [ ] Authentication based on a JWT token
