use std::fs::File;
use std::io::Read;
use axum::response::Html;
use axum::{routing::get, Router};
use sync_wrapper::SyncWrapper;

async fn html() -> Html<&'static str> {
    let mut f = File::open("src/index.html").expect("not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("oh no");

    let actual_str = String::from(&contents).to_string();
    let test = Box::leak(Box::new(actual_str));
    return Html(&test.as_str())

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