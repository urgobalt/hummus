use tokio::net::TcpListener;

use axum::Router;
use listenfd::ListenFd;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {}

#[tokio::main]
async fn main() {
    let state = AppState {};
    let router = Router::new()
        .with_state(state)
        .nest("/api", api::router())
        .route_service("/", ServeFile::new("../ui/dist/index.html"))
        .nest_service("/assets", ServeDir::new("../ui/dist/assets"));

    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
