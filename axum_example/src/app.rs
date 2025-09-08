use crate::command::CommandExecutor;
use crate::command::TokioCommandExecutor;
use axum::extract::State;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    command_executor: Arc<dyn CommandExecutor + Send + Sync + 'static>,
}

impl AppState {
    fn new(command_executor: Arc<dyn CommandExecutor + Send + Sync + 'static>) -> Self {
        Self { command_executor }
    }
}

async fn hello(State(state): State<AppState>) -> String {
    state.command_executor.execute("echo Hello, Axum!").await
}

pub(crate) async fn build_app() -> Router {
    let command_executor =
        Arc::new(TokioCommandExecutor) as Arc<dyn CommandExecutor + Send + Sync + 'static>;

    Router::new()
        .route("/hello", get(hello))
        .with_state(AppState::new(command_executor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use mockall::mock;

    #[tokio::test]
    async fn test_route() -> Result<(), Box<dyn std::error::Error>> {
        let server = TestServer::new(build_app().await)?;
        let response = server.get("/hello").await;
        assert_eq!(response.text(), "Hello, Axum!\n");
        Ok(())
    }

    mock! {
        pub CommandExecutor {

        }

        #[async_trait::async_trait]
        impl CommandExecutor for CommandExecutor {
            async fn execute(&self, command: &str) -> String;
        }
    }

    async fn build_app_mocked() -> Router {
        let mut mock_executor = MockCommandExecutor::new();
        mock_executor
            .expect_execute()
            .withf(|cmd| cmd == "echo Hello, Axum!")
            .returning(|_| "Hello, Axum!\n".to_string());

        Router::new()
            .route("/hello", get(hello))
            .with_state(AppState::new(Arc::new(mock_executor)))
    }

    #[tokio::test]
    async fn test_route_mocked() -> Result<(), Box<dyn std::error::Error>> {
        let server = TestServer::new(build_app_mocked().await)?;
        let response = server.get("/hello").await;
        assert_eq!(response.text(), "Hello, Axum!\n");
        Ok(())
    }
}
