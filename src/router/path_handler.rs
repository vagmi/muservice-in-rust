use axum::extract::Path;

use crate::greeting::greet;

pub async fn say_hello(Path(name): Path<String>) -> String {
    greet(&name)
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
