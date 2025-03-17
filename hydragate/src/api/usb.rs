use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use common::state::{UsbState, UsbType};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, broadcast::Sender};

use crate::{
    Result,
    db::DbState,
    error::{AppError, Error},
    util,
};

use super::ApiResult;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(setting))
        .route("/scan", post(scan))
        .route("/stop", post(stop))
}

async fn index(Extension(usb_state): Extension<Arc<Mutex<UsbState>>>) -> impl IntoResponse {
    #[derive(Debug, Serialize)]
    struct UsbDevice {
        usb_serial: String,
        ussb_num: String,
    }

    let usb_state = usb_state.lock().await;

    let data = usb_state
        .usb_devices
        .iter()
        .map(|(usb_serial, usb_num)| UsbDevice {
            usb_serial: usb_serial.clone(),
            ussb_num: usb_num.clone(),
        })
        .collect::<Vec<_>>();

    Json(data)
}

#[derive(Debug, Deserialize)]
struct PostSetting {
    usb_type: i64,
}
async fn setting(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
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

async fn scan(
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    body: String,
) -> impl IntoResponse {
    if let Ok(post_data) = serde_json::from_str::<PostSetting>(&body) {
        let usb_type = UsbType::from(post_data.usb_type);
        let mut usb_state = usb_state.lock().await;
        usb_state.usb_type = Some(usb_type);

        let serials = util::find_usb_driver(usb_type);
        let usb_devices = util::find_usb_number(usb_type, &serials);
        usb_state.usb_devices = usb_devices;
    }

    Json(ApiResult::OK)
}

async fn stop(Extension(usb_state): Extension<Arc<Mutex<UsbState>>>) -> impl IntoResponse {
    let mut usb_state = usb_state.lock().await;
    usb_state.usb_type = None;
    usb_state.usb_devices.clear();

    Json(ApiResult::OK)
}
