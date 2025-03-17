use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    response::IntoResponse,
    routing::{get, post},
};
use chrono::Local;
use common::state::{OrbbecPlace, U2d2Place, UsbState, UsbType};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, broadcast::Sender};

use crate::{
    Result,
    api::DataList,
    db::DbState,
    error::{AppError, Error},
    model::setting::TABLE_SETTING,
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

    Json(DataList { data })
}
#[derive(Debug, Deserialize)]
struct PostSetting {
    u2d2_left: Option<String>,
    u2d2_right: Option<String>,
    orbbec_head: Option<String>,
    orbbec_left: Option<String>,
    orbbec_right: Option<String>,
}

async fn setting(
    Extension(app_state): Extension<Arc<DbState>>,
    body: String,
) -> Result<impl IntoResponse> {
    let Ok(post_data) = serde_json::from_str::<PostSetting>(&body) else {
        return Err(Error::App(AppError::JSON_PARSE_FAILED));
    };

    let mut datas = vec![];

    if let Some(u2d2_left) = post_data.u2d2_left {
        datas.push((UsbType::U2D2 as i64, U2d2Place::Left as i64, u2d2_left));
    }
    if let Some(u2d2_right) = post_data.u2d2_right {
        datas.push((UsbType::U2D2 as i64, U2d2Place::Right as i64, u2d2_right));
    }
    if let Some(orbbec_head) = post_data.orbbec_head {
        datas.push((
            UsbType::Orbbec as i64,
            OrbbecPlace::Head as i64,
            orbbec_head,
        ));
    }
    if let Some(orbbec_left) = post_data.orbbec_left {
        datas.push((
            UsbType::Orbbec as i64,
            OrbbecPlace::Left as i64,
            orbbec_left,
        ));
    }
    if let Some(orbbec_right) = post_data.orbbec_right {
        datas.push((
            UsbType::Orbbec as i64,
            OrbbecPlace::Left as i64,
            orbbec_right,
        ));
    }

    if !datas.is_empty() {
        for (usb_type, place, serial) in datas.iter() {
            let rows_affected = sqlx::query(
                format!(
                    "UPDATE `{}` SET device_name = ? WHERE device_type = ? AND device_place = ?;",
                    TABLE_SETTING
                )
                .as_str(),
            )
            .bind(serial)
            .bind(usb_type)
            .bind(place)
            .execute(&app_state.database)
            .await
            .or(Err(Error::App(AppError::DB_ERROR)))?
            .rows_affected();

            if rows_affected == 0 {
                sqlx::query(format!("INSERT INTO `{}` (device_type, device_place, device_name, device_serial, created_at) VALUES (?, ?, ?, ?, ?);", TABLE_SETTING).as_str())
                .bind(usb_type)
                .bind(place)
                .bind(serial)
                .bind(serial)
                .bind(Local::now().timestamp())
                .execute(&app_state.database)
                .await
                .or(Err(Error::App(AppError::DB_ERROR)))?;
            }
        }
    }

    Ok(Json(ApiResult::OK))
}
#[derive(Debug, Deserialize)]
struct PostScan {
    usb_type: i64,
}

async fn scan(
    Extension(usb_state): Extension<Arc<Mutex<UsbState>>>,
    body: String,
) -> impl IntoResponse {
    if let Ok(post_data) = serde_json::from_str::<PostScan>(&body) {
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
