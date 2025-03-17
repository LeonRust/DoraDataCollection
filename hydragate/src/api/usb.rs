use std::{sync::Arc, time::Duration};

use axum::{
    Extension, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use common::state::{UsbState, UsbType};
use serde::Deserialize;
use tokio::sync::{Mutex, broadcast::Sender};

use crate::{
    Result,
    db::DbState,
    error::{AppError, Error},
    util,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(setting))
        .route("/stop", post(stop))
}

async fn index(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    Extension(set_usb_event): Extension<Arc<Sender<UsbType>>>,
) -> impl IntoResponse {
    Json(())
}

#[derive(Debug, Deserialize)]
struct PostSetting {
    usb_type: i64,
}
async fn setting(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    Extension(set_usb_event): Extension<Arc<Sender<UsbType>>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<PostSetting>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let usb_type = UsbType::from(post_data.usb_type);

    let mut usb_state = usb_state.lock().await;
    usb_state.usb_type = Some(usb_type);

    Ok(Json(()))
}

async fn stop(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    Extension(set_usb_event): Extension<Arc<Sender<UsbType>>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<PostSetting>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let usb_type = UsbType::from(post_data.usb_type);

    // usb订阅
    set_usb_event.send(usb_type).ok();

    Ok(Json(()))
}
