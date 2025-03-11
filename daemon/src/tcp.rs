use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};

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

    let global_atom_timestamp = Arc::new(AtomicI64::default());

    let global_atom_timestamp_clone = global_atom_timestamp.clone();
    tokio::spawn(async move {
        loop {
            global_atom_timestamp_clone.store(Local::now().timestamp_millis(), Ordering::Release);
            sleep(Duration::from_millis(clock_interval)).await;
        }
    });

    loop {
        let (mut socket, _) = listener.accept().await?;

        let tcp_state_clone = tcp_state.clone();
        let global_atom_timestamp_clone = global_atom_timestamp.clone();
        tokio::spawn(async move {
            let mut timestamp = Local::now().timestamp_millis();
            loop {
                // eprintln!("{:?}", socket);
                let golbal_timestamp = global_atom_timestamp_clone.load(Ordering::Relaxed);

                if timestamp != golbal_timestamp {
                    timestamp = golbal_timestamp;

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
                sleep(Duration::from_millis(10)).await;
            }
        });
    }

    // Ok(())
}
