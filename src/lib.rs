use axum::response::Html;
use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;
use std::fs;

const PATH: &'static str = "./index.html";

async fn html() -> Html<&'static str> {
    let content = fs::read_to_string(PATH).unwrap();
    let test = Box::leak(Box::new(content));
    return Html(&test.as_str())
}

#[shuttle_service::main]
async fn axum() -> shuttle_service::ShuttleAxum {
    let router = Router::new().route("/", get(html));
    let sync_wrapper = SyncWrapper::new(router);
    Ok(sync_wrapper)
}