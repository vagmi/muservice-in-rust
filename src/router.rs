use axum::{Router, routing::{get, post}};
use tower_http::trace::TraceLayer;

use crate::{app::DbApp, error::Result};

use self::{ 
    path_handler::say_hello, 
    query_handler::handle_query, 
    json_handler::use_todo, tasks_api::{get_tasks, create_task}
};

mod path_handler;
mod query_handler;
mod json_handler;
mod tasks_api;

pub async fn create_router() -> Result<Router> {
    let app = DbApp::new().await?;
    let router = Router::new()
        .route("/hello/:name", get(say_hello))
        .route("/search", get(handle_query))
        .route("/todo/use", post(use_todo))
        .route("/api/tasks", get(get_tasks))
        .route("/api/tasks", post(create_task))
        .layer(TraceLayer::new_for_http())
        .with_state(app);

     Ok(router)   
}

