use std::{sync::Arc, time::Duration};

use chrono::Local;
use common::state::TcpState;
use tokio::{io::AsyncWriteExt, net::TcpListener, sync::Mutex, time::sleep};

pub async fn run(
    addr: String,
    clock_interval: u64,
    tcp_state: Arc<Mutex<TcpState>>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    eprintln!("Server has started, listen on {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        let tcp_state_clone = tcp_state.clone();
        tokio::spawn(async move {
            loop {
                // eprintln!("{:?}", socket);
                let timestamp = Local::now().timestamp_millis();
                sleep(Duration::from_millis(clock_interval)).await;

                let state = tcp_state_clone.lock().await;

                if let Err(e) = socket
                    .write_all(
                        &serde_json::to_vec(&TcpState {
                            biz_type: state.biz_type,
                            timestamp,
                            path: state.path.clone(),
                        })
                        .unwrap(),
                    )
                    .await
                {
                    eprintln!("Error writing to socket: {}", e);
                    break;
                }
            }
        });
    }

    // Ok(())
}
