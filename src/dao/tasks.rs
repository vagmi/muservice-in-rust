use async_trait::async_trait;
use chrono::{Utc, DateTime};
use serde::Serialize;
use sqlx::{Transaction, Postgres, FromRow};

use super::error::DaoError;

#[derive(FromRow, Debug, Serialize)]
struct Task {
    id: i64,
    title: String,
    done: bool,
    created_at: DateTime<Utc>
}

struct NewTaskRequest {
    title: String
}

#[async_trait]
trait TasksRepository {
    async fn get_tasks(&mut self) -> Result<Vec<Task>, DaoError>;
    async fn create_task(&mut self, task_req: NewTaskRequest) -> Result<Task, DaoError>;
}

struct DbTaskRepository<'a, 'c> {
    tx: &'a mut Transaction<'c, Postgres>
}

#[async_trait]
impl<'a, 'c> TasksRepository for DbTaskRepository<'a, 'c> {
    async fn get_tasks(&mut self) -> Result<Vec<Task>, DaoError> {
        let tasks = sqlx::query_as!(Task, "select * from tasks")
                          .fetch_all(&mut *self.tx).await?;
        Ok(tasks)
    }

    async fn create_task(&mut self, task_req: NewTaskRequest) -> Result<Task, DaoError> {
        let task = sqlx::query_as!(Task, 
                                   "insert into tasks(title) values($1) returning *",
                                   &task_req.title)
            .fetch_one(&mut *self.tx)
            .await?;
        Ok(task)
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use super::{DbTaskRepository, TasksRepository, NewTaskRequest};

    #[sqlx::test]
    async fn should_get_tasks(pool: PgPool) {
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("insert into tasks(title) values('first')")
             .execute(&mut tx).await.unwrap();
        let mut repo = DbTaskRepository {tx: &mut tx};
        let tasks = repo.get_tasks().await.unwrap();
        assert_eq!(1, tasks.len());
        assert_eq!(Some(&String::from("first")), 
                   tasks.first().map(|t| &t.title));
    }

    #[sqlx::test]
    async fn should_create_task(pool: PgPool) {
        let mut tx = pool.begin().await.unwrap();
        let mut repo = DbTaskRepository {tx: &mut tx};
        let task = repo.create_task(
            NewTaskRequest{title: String::from("learn rust")}
            ).await.unwrap();
        assert_eq!(String::from("learn rust"), task.title);
        assert_eq!(false, task.done);
    }
}
