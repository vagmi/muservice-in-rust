use axum::{extract::{self, State}, Json};
use hyper::StatusCode;

use crate::{app::DbApp, dao::tasks::{Task, DbTasksRepository, TasksRepository, NewTaskRequest}};

pub async fn get_tasks(State(app): State<DbApp>) -> Result<Json<Vec<Task>>, StatusCode> {
    let mut tx = app.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut repo = DbTasksRepository::new(&mut tx);
    let tasks = repo.get_tasks().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(tasks))
}

pub async fn create_task(State(app): State<DbApp>, 
                         extract::Json(task_req): extract::Json<NewTaskRequest>) -> Result<Json<Task>, StatusCode> {
    let mut tx = app.pool.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut repo = DbTasksRepository::new(&mut tx);
    let task = repo.create_task(task_req).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(task))
}
