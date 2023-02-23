use axum::extract::Query;
use axum_macros::debug_handler;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct SearchQuery {
    q: String
}

#[debug_handler]
pub async fn handle_query(Query(params): Query<SearchQuery>) -> String {
    format!("Let me search for {}", params.q)
}

#[cfg(test)]
mod tests {
    use super::{SearchQuery, handle_query};
    use axum::extract::Query;

    #[tokio::test]
    async fn should_process_query_param() {
        let q: String = String::from("search_term");
        assert_eq!(String::from("Let me search for search_term"),
                   handle_query(Query(SearchQuery{q})).await);
    }
}
