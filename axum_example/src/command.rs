use async_trait::async_trait;

#[async_trait]
pub(crate) trait CommandExecutor {
    async fn execute(&self, command: &str) -> String;
}

pub(crate) struct TokioCommandExecutor;

#[async_trait]
impl CommandExecutor for TokioCommandExecutor {
    async fn execute(&self, command: &str) -> String {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let output = tokio::process::Command::new(parts[0])
            .args(parts[1..].into_iter())
            .output()
            .await
            .expect("command failed to run");
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
