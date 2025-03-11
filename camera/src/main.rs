use std::{env, path::PathBuf, time::Duration};

use common::{
    config,
    state::{BizType, PathSync, TcpState},
};
use opencv::{core::Vector, imgcodecs, prelude::*, videoio};
use tokio::{
    fs,
    io::{self, AsyncReadExt},
    net::TcpStream,
    sync::mpsc,
    time::sleep,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Daemon server info
    let daemon_ip = env::var(config::DAEMON_IP).unwrap_or("127.0.0.1".to_string());
    let daemon_tcp_port = env::var(config::DEAMON_TCP_PORT)
        .map(|v| v.parse().unwrap_or(1234))
        .unwrap_or(1234_u16);

    // 摄像头id opencv使用
    let camera_id = env::var(config::CAMERA_ID)
        .map(|v| v.parse().unwrap_or(0))
        .unwrap_or_default();
    // 摄像头编号 保存目录使用
    let camera_number = env::var(config::CAMERA_NUMBER).unwrap_or("1".to_string());

    let mut cam = videoio::VideoCapture::new(camera_id, videoio::CAP_ANY).unwrap();
    let opened = videoio::VideoCapture::is_opened(&cam).unwrap();
    if !opened {
        // 无法打开摄像头
        panic!("Unable to open camera{}!", camera_id);
    }

    let (img_sender, mut img_receiver) = mpsc::channel::<(PathBuf, Mat)>(100);

    tokio::spawn(async move {
        while let Some(img) = img_receiver.recv().await {
            let mut params = Vector::new();
            params.push(imgcodecs::IMWRITE_JPEG_QUALITY);
            params.push(70); // 压缩
            let _ = imgcodecs::imwrite(img.0.to_str().unwrap(), &img.1, &params);
        }
    });

    // TCP socket
    let tcp_addr = format!("{}:{}", daemon_ip, daemon_tcp_port);
    loop {
        match TcpStream::connect(&tcp_addr).await {
            Ok(socket) => {
                let (mut rd, _) = io::split(socket);
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
                                        let camera_path =
                                            format!("{}/cam-{}", tcp_state.path, camera_number);
                                        fs::create_dir_all(&camera_path).await?;
                                        eprintln!("Start None create dir");

                                        path = PathSync::Path(camera_path);
                                    }
                                }
                                PathSync::Path(ref current_path) => {
                                    if !current_path.is_empty() {
                                        let camera_path =
                                            format!("{}/cam-{}", tcp_state.path, camera_number);

                                        if current_path != &camera_path {
                                            fs::create_dir_all(&camera_path).await?;
                                            eprintln!("Start path create dir");

                                            path = PathSync::Path(camera_path);
                                        } else if timestamp != tcp_state.timestamp {
                                            let mut frame = Mat::default();
                                            if cam.read(&mut frame).is_ok()
                                                && frame.size().unwrap().width > 0
                                            {
                                                let filename = PathBuf::from(current_path)
                                                    .join(format!("{}.jpg", tcp_state.timestamp));

                                                let _ = img_sender.send((filename, frame)).await;
                                            }
                                            timestamp = tcp_state.timestamp;
                                        }
                                    }
                                }
                            }
                        }
                        eprintln!("Camera data {:?}", tcp_state);
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
