use axum::Router;
use serde::Serialize;

pub mod episode;
pub mod login;
pub mod robot;
pub mod scene;
pub mod task;
pub mod usb;

pub fn router() -> Router {
    Router::new()
        .nest("/login", login::router())
        .nest("/robot", robot::router())
        .nest("/scene", scene::router())
        .nest("/task", task::router())
        .nest("/episode", episode::router())
        .nest("/usb", usb::router())
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiResult {
    pub result: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiId {
    pub id: i64,
}

#[derive(Debug, Serialize)]
pub(crate) struct DataList<T: Serialize> {
    pub data: Vec<T>,
}
