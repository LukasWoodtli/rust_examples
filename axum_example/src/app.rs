use axum::Router;
use axum::routing::get;
use crate::command;

async fn hello() -> String {
    command::run_shell_command().await
}

pub(crate) async fn build_app() -> Router {
    Router::new().route("/hello", get(hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_route() -> Result<(), Box<dyn std::error::Error>> {
        let server = TestServer::new(build_app().await)?;
        let response = server.get("/hello").await;
        assert_eq!(response.text(), "Hello, Axum!\n");
        Ok(())
    }
}