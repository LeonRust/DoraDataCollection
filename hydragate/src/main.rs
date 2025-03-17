use std::{
    collections::BTreeMap,
    env, fs,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI64, AtomicU32},
    },
};

use common::{
    config,
    state::{BizType, TcpState, UsbState},
};
use db::DbState;
use tokio::sync::Mutex;

mod api;
mod asset;
mod db;
mod error;
mod http;
mod keyboard;
mod model;
mod tcp;
mod util;

pub use error::Result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // databases collect save path
    let datasets_path = env::var(config::DATASET_PATH).unwrap_or("datasets".to_string());
    fs::create_dir_all(&datasets_path).expect("datasets floder created fail");

    // Daemon server info
    // let daemon_ip = env::var(config::DAEMON_IP).unwrap_or("127.0.0.1".to_string());
    let daemon_tcp_port = env::var(config::DAEMON_TCP_PORT)
        .map(|v| v.parse().unwrap_or(1234))
        .unwrap_or(1234_u16);
    let daemon_http_port = env::var(config::DAEMON_HTTP_PORT)
        .map(|v| v.parse().unwrap_or(7878))
        .unwrap_or(7878_u16);

    // clock interval
    let clock_interval = env::var(config::CLOCK_INTERVAL)
        .map_or(config::DEFAULT_CLOCK_INTERVAL, |v| {
            v.parse().unwrap_or(config::DEFAULT_CLOCK_INTERVAL)
        });

    // database
    let database = db::db_connect().await?;

    // global data
    let db_state = Arc::new(DbState {
        database,
        data_id: AtomicI64::default(),
        robot_id: AtomicU32::default(),
        scene_id: AtomicU32::default(),
        task_id: AtomicU32::default(),
        episode_id: AtomicU32::default(),
        show_result: AtomicBool::default(),
    });
    let tcp_state = Arc::new(Mutex::new(TcpState {
        biz_type: BizType::None,
        timestamp: 0,
        path: String::new(),
    }));
    let usb_state = Arc::new(Mutex::new(UsbState {
        usb_type: None,
        usb_devices: BTreeMap::new(),
        u2d2_left: None,
        u2d2_right: None,
        orbbec_head: None,
        orbbec_left: None,
        orbbec_right: None,
    }));

    // TCP
    let tcp_addr = format!(":::{}", daemon_tcp_port);
    let tcp_handel = tokio::spawn(tcp::run(tcp_addr, clock_interval, tcp_state.clone()));

    // HTTP
    let http_addr = format!(":::{}", daemon_http_port);
    let http_handel = tokio::spawn(http::run(
        http_addr,
        db_state.clone(),
        tcp_state.clone(),
        usb_state.clone(),
        datasets_path.clone(),
    ));

    // Keyboard
    let keyboard_handel = tokio::spawn(keyboard::run(tcp_state, db_state, datasets_path));

    // usb devices
    let usb_handle = tokio::spawn(async move {
        loop {
            let mut usb_state = usb_state.lock().await;
            if let Some(usb_type) = usb_state.usb_type {
                let serials = util::find_usb_driver(usb_type);
                let usb_devices = util::find_usb_number(usb_type, &serials);
                usb_state.usb_devices = usb_devices;
            }
            // println!("usb_devices: {:?}", usb_state.usb_devices);
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    });

    tokio::select! {
        _ = tcp_handel => {
            println!("TCP server stopped");
        }
        _ = http_handel => {
            println!("HTTP server stopped");
        }
        _ = keyboard_handel => {
            println!("Keyboard server stopped");
        }
        _ = usb_handle => {
            println!("USB server stopped");
        }
    }

    Ok(())
}
