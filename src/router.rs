use axum::{Router, routing::{get, post}};
use tower_http::trace::TraceLayer;

use self::{ 
    path_handler::say_hello, 
    query_handler::handle_query, 
    json_handler::use_todo
};

mod path_handler;
mod query_handler;
mod json_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/hello/:name", get(say_hello))
        .route("/search", get(handle_query))
        .route("/todo/use", post(use_todo))
        .layer(TraceLayer::new_for_http())
}

