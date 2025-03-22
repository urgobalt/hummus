use axum::Router;
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/hello", get(async || "hello"))
}
