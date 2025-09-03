
use axum::routing::get;
use axum::Router;

#[tokio::main(flavor = "current_thread")]
async fn main(){
    let app = Router::new().route("/", get(|| async { "Hello, Axum!\n" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
