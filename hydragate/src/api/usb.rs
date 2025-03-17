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
        .route("/stop", post(setting))
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

    // usb订阅
    let mut usb_event = set_usb_event.subscribe();

    let usb_state_clone = usb_state.clone();
    let usb_state_clone2 = usb_state.clone();
    tokio::spawn(async move {
        let setting = tokio::spawn(async move {
            let mut usb_state = usb_state_clone.lock().await;
            usb_state.setting = true;
            loop {
                let serials = util::find_usb_driver(usb_type);
                let usb_devices = util::find_usb_number(usb_type, serials);
                println!("usb_devices: {:?}", usb_devices);
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        });
        tokio::select! {
            _ = setting => println!("setting task exit"),
            Ok(_) = usb_event.recv() => {
                let mut usb_state = usb_state_clone2.lock().await;
                usb_state.setting = false;
                usb_state.serials = vec![];
            }
        }
    });

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
