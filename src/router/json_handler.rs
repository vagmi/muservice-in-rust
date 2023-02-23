use serde::{Deserialize, Serialize};
use axum::{extract, Json};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    title: String,
    done: bool
}

pub async fn use_todo(extract::Json(todo): extract::Json<Todo>) -> Json<Value> {
    Json(json!({
        "msg": format!("received todo - {}", todo.title)
    }))
}

#[cfg(test)]
mod tests {
    use axum::{extract, Json};
    use serde_json::Value;

    use super::{Todo, use_todo};

    #[tokio::test]
    async fn should_use_todo() {
        let body = Todo{
            title: String::from("learn rust"),
            done: true
        };
        let Json(val) = use_todo(extract::Json(body)).await;
        let expected_value = Value::String(String::from("received todo - learn rust"));
        
        assert_eq!(Some(&expected_value), val.get("msg"))
    }
}
