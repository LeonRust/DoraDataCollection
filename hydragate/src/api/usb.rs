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
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index).post(setting))
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

async fn stop(
    Extension(app_state): Extension<Arc<DbState>>,
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    Extension(set_usb_event): Extension<Arc<Sender<UsbType>>>,
) -> impl IntoResponse {
    let mut usb_state = usb_state.lock().await;
    usb_state.usb_type = None;
    usb_state.usb_devices.clear();

    Json(())
}
