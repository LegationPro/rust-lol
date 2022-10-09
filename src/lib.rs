use axum::response::Html;
use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn html() -> Html<&'static str> {
    Html(include_str!("./index.html"))
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/", get(html)).route("/hi", get(hello_world));
    let sync_wrapper = SyncWrapper::new(router);

    Ok(sync_wrapper)
}