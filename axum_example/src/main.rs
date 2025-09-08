mod app;
mod command;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = app::build_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
