use std::{env, sync::Arc, time::Duration};

use arm_state::ArmState;
use common::{
    config,
    state::{BizType, PathSync, TcpState},
};
use futures_util::StreamExt;
use tokio::{
    fs::{self, OpenOptions},
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
    time::sleep,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

mod arm_state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Daemon server info
    let daemon_ip = env::var(config::DAEMON_IP).unwrap_or("127.0.0.1".to_string());
    let daemon_tcp_port = env::var(config::DEAMON_TCP_PORT)
        .map(|v| v.parse().unwrap_or(1234))
        .unwrap_or(1234_u16);

    // arm direction left/right
    let mut arm_direction = env::var(config::ARM_DIRECTION).unwrap_or("right".to_string());
    if !["left", "right"].contains(&arm_direction.as_str()) {
        arm_direction = "right".to_string();
    }

    // arm info
    let arm_ip = env::var(config::ARM_IP).unwrap_or("192.168.1.18".to_string());
    let arm_url = format!("ws://{}:8060", arm_ip);

    let joint_position = Arc::new(Mutex::new(vec![]));
    let joint_position_clone = joint_position.clone();
    tokio::spawn(async move {
        loop {
            match connect_async(&arm_url).await {
                Ok((ws_stream, _)) => {
                    eprintln!("Connected");
                    let (_, mut read) = ws_stream.split();
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if let Ok(arm_state) =
                                    serde_json::from_slice::<ArmState>(text.as_bytes())
                                {
                                    // println!("{:?}", arm_state.joint_status.joint_position);
                                    *joint_position_clone.lock().await =
                                        arm_state.joint_status.joint_position;
                                }
                            }
                            _ => {
                                eprintln!("Disconnected");
                                // tokio::time::sleep(Duration::from_secs(5)).await;
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Can't connect. try again: {}", e);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });

    // TCP socket
    let tcp_addr = format!("{}:{}", daemon_ip, daemon_tcp_port);
    loop {
        match TcpStream::connect(&tcp_addr).await {
            Ok(socket) => {
                let (mut rd, _) = io::split(socket);

                let mut arm_file_all: Option<fs::File> = None;
                let mut arm_file_0: Option<fs::File> = None;
                let mut arm_file_1: Option<fs::File> = None;
                let mut arm_file_2: Option<fs::File> = None;
                let mut arm_file_3: Option<fs::File> = None;
                let mut arm_file_4: Option<fs::File> = None;
                let mut arm_file_5: Option<fs::File> = None;
                let mut arm_file_6: Option<fs::File> = None;

                let mut buf = [0; 1024];
                let mut timestamp = 0;

                // 路径同步
                let mut path = PathSync::None;

                loop {
                    let n = rd.read(&mut buf).await?;

                    if n == 0 {
                        break;
                    }

                    if let Ok(ref tcp_state) = serde_json::from_slice::<TcpState>(&buf[..n]) {
                        if tcp_state.biz_type == BizType::Start {
                            match path {
                                PathSync::None => {
                                    if !tcp_state.path.is_empty() {
                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            None,
                                            &mut arm_file_all,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(0),
                                            &mut arm_file_0,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(1),
                                            &mut arm_file_1,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(2),
                                            &mut arm_file_2,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(3),
                                            &mut arm_file_3,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(4),
                                            &mut arm_file_4,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(5),
                                            &mut arm_file_5,
                                        )
                                        .await;

                                        create_file(
                                            &tcp_state.path,
                                            &arm_direction,
                                            Some(6),
                                            &mut arm_file_6,
                                        )
                                        .await;

                                        path = PathSync::Path(tcp_state.path.clone());
                                    }
                                }
                                PathSync::Path(ref current_path) => {
                                    if !current_path.is_empty() {
                                        if current_path != &tcp_state.path {
                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                None,
                                                &mut arm_file_all,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(0),
                                                &mut arm_file_0,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(1),
                                                &mut arm_file_1,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(2),
                                                &mut arm_file_2,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(3),
                                                &mut arm_file_3,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(4),
                                                &mut arm_file_4,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(5),
                                                &mut arm_file_5,
                                            )
                                            .await;

                                            create_file(
                                                &tcp_state.path,
                                                &arm_direction,
                                                Some(6),
                                                &mut arm_file_6,
                                            )
                                            .await;

                                            path = PathSync::Path(tcp_state.path.clone());
                                        } else if timestamp != tcp_state.timestamp {
                                            let data = joint_position.lock().await;

                                            if let Some(ref mut file) = arm_file_all {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {} {} {} {} {} {} {}\n",
                                                            tcp_state.timestamp,
                                                            data[0],
                                                            data[1],
                                                            data[2],
                                                            data[3],
                                                            data[4],
                                                            data[5],
                                                            data[6]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_0 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[0]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_1 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[1]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_2 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[2]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_3 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[3]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_4 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[4]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_5 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[5]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            if let Some(ref mut file) = arm_file_6 {
                                                let _ = file
                                                    .write_all(
                                                        format!(
                                                            "{} {}\n",
                                                            tcp_state.timestamp, data[6]
                                                        )
                                                        .as_bytes(),
                                                    )
                                                    .await;
                                            }

                                            timestamp = tcp_state.timestamp;
                                        }
                                    }
                                }
                            }
                        }
                        eprintln!("data {:?}", tcp_state);
                    }

                    // let data = String::from_utf8_lossy(&buf[..n]);
                    // eprintln!("data {}", data);
                }
            }
            Err(e) => {
                eprintln!("TCP disconnected: {}", e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

async fn create_file(
    path: &str,
    arm_direction: &str,
    file_name: Option<u8>,
    file: &mut Option<fs::File>,
) {
    *file = Some(
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(format!(
                "{path}/{arm_direction}_master_{}.txt",
                if let Some(id) = file_name {
                    format!("arm_joint-{id}")
                } else {
                    "gripper".to_string()
                }
            ))
            .await
            .unwrap(),
    );
}
