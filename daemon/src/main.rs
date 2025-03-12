use std::{
    env, fs,
    sync::{
        atomic::{AtomicBool, AtomicI64, AtomicU32},
        Arc,
    },
};

use common::{
    config,
    state::{BizType, TcpState},
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

pub use error::Result;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // databases collect save path
    let datasets_path = env::var(config::DATASET_PATH).unwrap_or("datasets".to_string());
    fs::create_dir_all(&datasets_path).expect("datasets floder created fail");

    // Daemon server info
    let daemon_ip = env::var(config::DAEMON_IP).unwrap_or("127.0.0.1".to_string());
    let daemon_tcp_port = env::var(config::DEAMON_TCP_PORT)
        .map(|v| v.parse().unwrap_or(1234))
        .unwrap_or(1234_u16);
    let daemon_http_port = env::var(config::DEAMON_HTTP_PORT)
        .map(|v| v.parse().unwrap_or(80))
        .unwrap_or(80_u16);

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

    // TCP
    let tcp_addr = format!("{}:{}", daemon_ip, daemon_tcp_port);
    let tcp_handel = tokio::spawn(tcp::run(tcp_addr, clock_interval, tcp_state.clone()));

    // HTTP
    let http_addr = format!("{}:{}", daemon_ip, daemon_http_port);
    let http_handel = tokio::spawn(http::run(
        http_addr,
        db_state.clone(),
        tcp_state.clone(),
        datasets_path.clone(),
    ));

    // Keyboard
    let keyboard_handel = tokio::spawn(keyboard::run(tcp_state, db_state, datasets_path));

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
    }

    Ok(())
}
