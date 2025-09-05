use crate::command::{TokioCommandExecutor};
use std::sync::Arc;
use axum::{ Router};
use axum::extract::{State};
use axum::routing::get;
use crate::command::{CommandExecutor};

#[derive(Clone)]
struct AppState {
    command_executor: Arc<dyn CommandExecutor + Send + Sync + 'static>,
}

impl AppState {
    fn new(command_executor: Arc<dyn CommandExecutor + Send + Sync + 'static>) -> Self {
        Self { command_executor }
    }
}


async fn hello(
    State(state): State<AppState>) -> String {
    state.command_executor.execute("echo Hello, Axum!").await
}

pub(crate) async fn build_app() -> Router {
    let command_executor = Arc::new(TokioCommandExecutor) as Arc<dyn CommandExecutor + Send + Sync + 'static>;

    Router::new()
        .route("/hello", get(hello))
        .with_state(AppState::new(command_executor))
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
