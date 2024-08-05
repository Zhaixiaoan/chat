mod sse;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use sse::see_handler;

const INDEX_HTML: &str = include_str!("../index.html");
pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/events", get(see_handler))
}

async fn index_handler() -> impl IntoResponse {
    Html(INDEX_HTML)
}
