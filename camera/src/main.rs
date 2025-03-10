use std::{env, time::Duration};

use common::{config, state::TcpState};
use tokio::{
    io::{self, AsyncReadExt},
    net::TcpStream,
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

    // TCP socket
    let tcp_addr = format!("{}:{}", daemon_ip, daemon_tcp_port);
    loop {
        match TcpStream::connect(&tcp_addr).await {
            Ok(socket) => {
                let (mut rd, _) = io::split(socket);
                let mut buf = [0; 1024];

                loop {
                    let n = rd.read(&mut buf).await?;

                    if n == 0 {
                        break;
                    }

                    if let Ok(tcp_state) = serde_json::from_slice::<TcpState>(&buf[..n]) {
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
