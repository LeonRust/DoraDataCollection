use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpState {
    pub biz_type: BizType,
    pub timestamp: i64,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BizType {
    None,
    Start,
    Stop,
}
