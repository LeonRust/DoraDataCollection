use std::sync::Arc;

use common::state::{BizType, TcpState};
use device_query::{DeviceQuery, DeviceState, Keycode};
use tokio::sync::Mutex;

use crate::db::DbState;

pub async fn run(tcp_state: Arc<Mutex<TcpState>>, db_state: Arc<DbState>) -> anyhow::Result<()> {
    let device_state = DeviceState::new();

    let mut prev_keys = vec![];

    loop {
        let keys = device_state.get_keys();

        if keys != prev_keys {
            if let Some(key) = keys.first() {
                let mut state = tcp_state.lock().await;
                match key {
                    Keycode::Key1 | Keycode::Numpad1 => {
                        eprintln!("Start");
                        match state.biz_type {
                            BizType::None => {
                                // TODO add new spisode to db

                                state.biz_type = BizType::Start;
                            }
                            BizType::Start => {}
                            BizType::Stop => {
                                // TODO Save to db, success

                                state.biz_type = BizType::None;
                            }
                        }
                    }
                    Keycode::Key0 | Keycode::Numpad0 => {
                        eprintln!("Stop");
                        match state.biz_type {
                            BizType::None => {}
                            BizType::Start => state.biz_type = BizType::Stop,
                            BizType::Stop => {
                                // TODO Save to db, fail

                                state.biz_type = BizType::None;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        prev_keys = keys;
    }
}
