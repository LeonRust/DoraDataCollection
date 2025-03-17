use std::{process::Command, sync::Arc};

use axum::{Extension, Json, Router, response::IntoResponse, routing::post};
use common::state::UsbType;

use crate::{
    Result,
    db::DbState,
    error::{AppError, Error},
    model::setting::Setting,
    util,
};

use super::ApiResult;

pub fn router() -> Router {
    Router::new()
        .route("/run", post(run))
        .route("/stop", post(stop))
}

async fn run(Extension(db_state): Extension<Arc<DbState>>) -> Result<impl IntoResponse> {
    let settings = Setting::list(&db_state.database).await;
    if settings.is_empty() {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    }

    let mut u2d2_left = None; // 摇操臂左
    let mut u2d2_right = None; // 摇操臂右
    let mut orbbec_head = None; // 头部相机
    let mut orbbec_left = None; // 左侧相机
    let mut orbbec_right = None; // 右侧相机

    let u2d2_serials = util::find_usb_driver(UsbType::U2D2);
    let u2d2_usb_devices = util::find_usb_number(UsbType::U2D2, &u2d2_serials);
    let orbbec_serials = util::find_usb_driver(UsbType::Orbbec);
    let orbbec_usb_devices = util::find_usb_number(UsbType::Orbbec, &orbbec_serials);

    for setting in settings.iter() {
        match setting.device_type {
            1 => {
                // U2D2
                let Some(value) = u2d2_usb_devices.get(&setting.device_serial) else {
                    return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
                };

                match setting.device_place {
                    1 => {
                        // 左侧
                        u2d2_left = Some((setting.device_serial.to_string(), value.to_string()));
                    }
                    2 => {
                        // 右侧
                        u2d2_right = Some((setting.device_serial.to_string(), value.to_string()));
                    }
                    _ => {}
                }
            }
            2 => {
                // 奥比中光
                let Some(value) = orbbec_usb_devices.get(&setting.device_serial) else {
                    return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
                };
                match setting.device_place {
                    1 => {
                        // 头部
                        orbbec_head = Some((setting.device_serial.to_string(), value.to_string()));
                    }
                    2 => {
                        // 左侧
                        orbbec_left = Some((setting.device_serial.to_string(), value.to_string()));
                    }
                    3 => {
                        // 右侧
                        orbbec_right = Some((setting.device_serial.to_string(), value.to_string()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let Some(u2d2_left) = u2d2_left else {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    };
    let Some(u2d2_right) = u2d2_right else {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    };
    let Some(orbbec_head) = orbbec_head else {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    };
    let Some(orbbec_left) = orbbec_left else {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    };
    let Some(orbbec_right) = orbbec_right else {
        return Err(Error::App(AppError::RUN_SYSTEM_FAIL));
    };

    let mut tasks = vec![];
    // 启动lerobot-gen72
    let cache_path = db_state.cache_path.clone();
    tasks.push(tokio::spawn(async move {
        Command::new("sudo")
            .args([
                "docker",
                "run",
                "--rm",
                "-d",
                "--privileged",
                "--name",
                "lerobot-gen72",
                "--network",
                "host",
                "--device",
                format!("/dev/{}:/dev/ttyUSB0", u2d2_left.1).as_str(),
                "--device",
                format!("/dev/{}:/dev/ttyUSB1", u2d2_right.1).as_str(),
                "--cap-add",
                "SYS_PTRACE",
                "-e",
                "RIGHT_ARM_IP=192.168.1.19",
                "-e",
                "LEFT_ARM_IP=192.168.1.20",
                "-e",
                "RIGHT_ARM_PORT=/dev/ttyUSB0",
                "-e",
                "LEFT_ARM_PORT=/dev/ttyUSB1",
                "-e",
                "DLL_PATH=lerobot/common/robot_devices/robots/libs",
                "-v",
                format!("{}:/lerobot-gen72/.cache", cache_path).as_str(),
                "lerobot-gen72",
            ])
            .output()
            .ok();
    }));

    // 启动 head
    for (index, (_name, camera)) in [orbbec_head, orbbec_left, orbbec_right].iter().enumerate() {
        let datasets_path = db_state.datasets_path.clone();
        let camera = camera.clone();
        tasks.push(tokio::spawn(async move {
            Command::new("sudo")
                .args([
                    "docker",
                    "run",
                    "--rm",
                    "-d",
                    "--name",
                    format!(
                        "camera-{}",
                        if index == 0 {
                            "head"
                        } else if index == 1 {
                            "left"
                        } else {
                            "right"
                        }
                    )
                    .as_str(),
                    "--network",
                    "host",
                    "--device",
                    format!("/dev/{}:/dev/video0", camera).as_str(),
                    "-e",
                    "DAEMON_IP=127.0.0.1",
                    "-e",
                    "DAEMON_TCP_PORT=1234",
                    "-e",
                    "CAMERA_ID=0",
                    "-e",
                    format!("CAMERA_NUMBER={}", index + 1).as_str(),
                    "-v",
                    format!("{}:/datasets", datasets_path).as_str(),
                    "camera",
                ])
                .output()
                .ok();
        }));
    }

    futures::future::join_all(tasks).await;

    Ok(Json(ApiResult::OK))
}

async fn stop() -> impl IntoResponse {
    tokio::spawn(async move {
        Command::new("sudo")
            .args([
                "docker",
                "stop",
                "lerobot-gen72",
                "camera-head",
                "camera-left",
                "camera-right",
            ])
            .output()
            .ok();
    });

    Json(ApiResult::OK)
}
