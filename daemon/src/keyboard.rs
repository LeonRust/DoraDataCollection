use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, Mutex};

use common::state::{BizType, TcpState};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::db::DbState;

pub async fn run(
    tcp_state: Arc<Mutex<TcpState>>,
    db_state: Arc<DbState>,
    datasets_path: String,
) -> anyhow::Result<()> {
    // Create a channel for sending keypress events
    let (tx, mut rx) = mpsc::channel(32);

    // Spawn a blocking task to handle keyboard input
    std::thread::spawn(move || {
        let device_state = DeviceState::new();
        let mut prev_keys = vec![];

        loop {
            let keys = device_state.get_keys();
            if keys != prev_keys {
                if let Some(key) = keys.first() {
                    // Send the key through the channel
                    if tx.blocking_send(*key).is_err() {
                        break; // Channel closed, exit thread
                    }
                }
            }
            prev_keys = keys;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    // Process keyboard events in the async context
    while let Some(key) = rx.recv().await {
        let mut mut_tcp_state = tcp_state.lock().await;

        match key {
            Keycode::Key1 | Keycode::Numpad1 => {
                eprintln!("Start/Success pressed");
                match mut_tcp_state.biz_type {
                    BizType::None => {
                        db_state
                            .episode_id
                            .fetch_add(1, std::sync::atomic::Ordering::Release);

                        // TODO add new spisode to db

                        mut_tcp_state.biz_type = BizType::Start;
                    }
                    BizType::Start => {}
                    BizType::Stop => {
                        // TODO Save to db, success

                        eprintln!("Episode Sucess");

                        mut_tcp_state.biz_type = BizType::None;
                    }
                }
            }
            Keycode::Key0 | Keycode::Numpad0 => {
                eprintln!("Stop/Fail pressed");
                match mut_tcp_state.biz_type {
                    BizType::None => {}
                    BizType::Start => mut_tcp_state.biz_type = BizType::Stop,
                    BizType::Stop => {
                        // TODO Save to db, fail

                        eprintln!("Episode Fail");

                        mut_tcp_state.biz_type = BizType::None;
                    }
                }
            }
            Keycode::I => {
                eprintln!("Init");
                db_state
                    .robot_id
                    .store(1, std::sync::atomic::Ordering::Release);
                db_state
                    .scene_id
                    .store(1, std::sync::atomic::Ordering::Release);
                db_state
                    .task_id
                    .store(1, std::sync::atomic::Ordering::Release);
                db_state
                    .episode_id
                    .store(0, std::sync::atomic::Ordering::Release);
            }
            _ => {}
        }

        // path
        let episode_id = db_state
            .episode_id
            .load(std::sync::atomic::Ordering::Relaxed);
        if episode_id > 0 {
            let path = format!(
                "{datasets_path}/robot-{}/scene-{}/task-{}/episode-{}",
                db_state.robot_id.load(std::sync::atomic::Ordering::Relaxed),
                db_state.scene_id.load(std::sync::atomic::Ordering::Relaxed),
                db_state.task_id.load(std::sync::atomic::Ordering::Relaxed),
                db_state
                    .episode_id
                    .load(std::sync::atomic::Ordering::Relaxed)
            );
            if !PathBuf::from(&path).exists() {
                tokio::fs::create_dir_all(&path).await?;
            }
            mut_tcp_state.path = path;
        }
    }

    Ok(())
}

