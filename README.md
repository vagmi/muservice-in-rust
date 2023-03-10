# Building a microservice in rust

This project is a companion to my talk at ConFoo about building a rust project.

You should checkout a fully fleshed out starter template for building 
microservices at [tarkalabs/muservice-rs](https://github.com/tarkalabs/muservice-rs).

If you just cloned the repo you'd want to get to the step-0 branch.

```
git checkout step-0
```

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

* DAO - `./src/dao/mod.rs`
* Tasks - `./src/dao/tasks.rs`
* App State - `./src/app/mod.rs`
* API - `./src/router/tasks_api.rs`

Run `git checkout step-06` to get to the next step.

## Step 6

In this step we accept an Authorization header. If the authorization header
is not present or if it is incorrect we present a 401. If not we show the 
claims as a JSON object.

* Secure Route - `./src/router/auth_endpoint.rs`
* Include Secret - `./src/app/mod.rs` and `./src/router.rs`

```
http localhost:3000/api/secure Authorization:eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI0MiIsIm5hbWUiOiJWYWdtaSIsImlhdCI6MTUxNjIzOTAyMiwiZXhwIjoxODc3MjI0Njk2fQ.ePx-Gzd6-PsZ2vKn7v5QmpgAt8vQ8c5AgXLsHR1AFR0
```

* [x] Bootstrap Rust project
* [x] Add tokio and anyhow and run as an async binary
* [x] Library and binary structure and setup tracing
* [x] Add axum and setup a hello world endpoint
* [x] Explore different extractors for Path, Body and Query Params
* [x] Add database support with sqlx and postgres
* [x] Authentication based on a JWT token
