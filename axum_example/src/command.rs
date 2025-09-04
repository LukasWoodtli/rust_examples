
pub(crate) async fn run_shell_command() -> String {
    let mut child = tokio::process::Command::new("/usr/bin/echo");
    child.arg("Hello, Axum!");
    let stdout = child.stdout(std::process::Stdio::piped())
        .output()
        .await.expect("Failed to run echo command")
        .stdout;
    String::from_utf8_lossy(&stdout).to_string()
}
