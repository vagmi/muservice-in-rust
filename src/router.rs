use axum::{Router, extract::Path, routing::get};
use tower_http::trace::TraceLayer;
use crate::greeting::greet;


async fn say_hello(Path(name): Path<String>) -> String {
    greet(&name)
}

pub fn create_router() -> Router {
    Router::new()
        .route("/hello/:name", get(say_hello))
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;

    use super::say_hello;
    use crate::greeting::greet;

    #[tokio::test]
    async fn should_say_hello() {
        assert_eq!(greet("confoo-2023"), say_hello(Path("confoo-2023".to_string())).await)
    }
}
