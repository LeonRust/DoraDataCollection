use chrono::Local;
use std::{
    path::PathBuf,
    sync::{Arc, atomic::Ordering},
};
use tokio::{
    fs::File,
    io::AsyncWriteExt,
    sync::{Mutex, mpsc},
};

use common::state::{BizType, TcpState};
use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::{
    db::DbState,
    model::episode::{CreateEpisode, Episode, ResultEnum},
};

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
        let robot_id = db_state.robot_id.load(Ordering::Relaxed);
        let scene_id = db_state.scene_id.load(Ordering::Relaxed);
        let task_id = db_state.task_id.load(Ordering::Relaxed);

        match key {
            Keycode::Key1 | Keycode::Numpad1 => {
                // eprintln!("Start/Success pressed");
                match mut_tcp_state.biz_type {
                    BizType::None => {
                        if robot_id > 0 && scene_id > 0 && task_id > 0 {
                            db_state.episode_id.fetch_add(1, Ordering::Release);

                            // add new spisode to db
                            if let Ok(id) = Episode::create(
                                &db_state.database,
                                CreateEpisode {
                                    robot_id: db_state.robot_id.load(Ordering::Relaxed),
                                    scene_id: db_state.scene_id.load(Ordering::Relaxed),
                                    task_id: db_state.task_id.load(Ordering::Relaxed),
                                    episode_id: db_state.episode_id.load(Ordering::Relaxed),
                                },
                            )
                            .await
                            {
                                db_state.data_id.store(id, Ordering::Release);
                            }

                            mut_tcp_state.biz_type = BizType::Start;
                        }
                    }
                    BizType::Start => {}
                    BizType::Stop => {
                        // Save to db, success

                        // eprintln!("Episode Sucess");

                        let id = db_state.data_id.load(Ordering::Relaxed);
                        if id > 0 {
                            Episode::set_result(&db_state.database, ResultEnum::Success, id).await;
                            if let Ok(mut file) =
                                File::create(format!("{}/result.txt", mut_tcp_state.path)).await
                            {
                                file.write_all(
                                    format!("{} 1", Local::now().timestamp_millis()).as_bytes(),
                                )
                                .await
                                .ok();
                            }
                        }

                        mut_tcp_state.biz_type = BizType::None;
                        db_state.show_result.store(false, Ordering::Release);
                    }
                }
            }
            Keycode::Key0 | Keycode::Numpad0 => {
                // eprintln!("Stop/Fail pressed");
                match mut_tcp_state.biz_type {
                    BizType::None => {}
                    BizType::Start => {
                        mut_tcp_state.biz_type = BizType::Stop;

                        // 写入db
                        let id = db_state.data_id.load(Ordering::Relaxed);
                        if id > 0 {
                            Episode::stop_id(&db_state.database, id).await;
                        }

                        // 显示数据结果设置弹窗(成功/失败)
                        db_state.show_result.store(true, Ordering::Release);
                    }
                    BizType::Stop => {
                        // Save to db, fail

                        // eprintln!("Episode Fail");

                        let id = db_state.data_id.load(Ordering::Relaxed);
                        if id > 0 {
                            Episode::set_result(&db_state.database, ResultEnum::Fail, id).await;
                            if let Ok(mut file) =
                                File::create(format!("{}/result.txt", mut_tcp_state.path)).await
                            {
                                file.write_all(
                                    format!("{} 0", Local::now().timestamp_millis()).as_bytes(),
                                )
                                .await
                                .ok();
                            }
                        }

                        mut_tcp_state.biz_type = BizType::None;
                        db_state.show_result.store(false, Ordering::Release);
                    }
                }
            }
            // Keycode::I => {
            //     eprintln!("Init");
            //     db_state.robot_id.store(1, Ordering::Release);
            //     db_state.scene_id.store(1, Ordering::Release);
            //     db_state.task_id.store(1, Ordering::Release);
            //     db_state.episode_id.store(0, Ordering::Release);
            // }
            _ => {}
        }

        // path
        let episode_id = db_state.episode_id.load(Ordering::Relaxed);
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
