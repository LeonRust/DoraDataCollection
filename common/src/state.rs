use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpState {
    pub biz_type: BizType,
    pub timestamp: i64,
    pub path: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BizType {
    None,
    Start,
    Stop,
}

// 路径同步
pub enum PathSync {
    None,
    Path(String),
}

// 配置状态
pub struct UsbState {
    pub setting: bool, // 是否开始配置模式, 配置模式下可以配置U2D2和奥比中光相机
    pub serials: Vec<String>,
    pub u2d2_left: Option<(String, String)>,
    pub u2d2_right: Option<(String, String)>,
    pub orbbec_head: Option<(String, String)>,
    pub orbbec_left: Option<(String, String)>,
    pub orbbec_right: Option<(String, String)>,
}

// USB设备类型
#[derive(Debug, Clone, Copy)]
pub enum UsbType {
    U2D2,
    Orbbec,
}
impl From<i64> for UsbType {
    fn from(value: i64) -> Self {
        match value {
            1 => UsbType::U2D2,
            2 => UsbType::Orbbec,
            _ => UsbType::U2D2,
        }
    }
}

impl From<UsbType> for i64 {
    fn from(value: UsbType) -> Self {
        match value {
            UsbType::U2D2 => 1,
            UsbType::Orbbec => 2,
        }
    }
}

// U2D2位置
pub enum U2d2Place {
    Left,
    Right,
}

impl From<i64> for U2d2Place {
    fn from(value: i64) -> Self {
        match value {
            1 => U2d2Place::Left,
            2 => U2d2Place::Right,
            _ => U2d2Place::Left,
        }
    }
}

impl From<U2d2Place> for i64 {
    fn from(value: U2d2Place) -> Self {
        match value {
            U2d2Place::Left => 1,
            U2d2Place::Right => 2,
        }
    }
}

// 奥比中光位置
pub enum OrbbecPlace {
    Head,
    Left,
    Right,
}

impl From<i64> for OrbbecPlace {
    fn from(value: i64) -> Self {
        match value {
            1 => OrbbecPlace::Head,
            2 => OrbbecPlace::Left,
            3 => OrbbecPlace::Right,
            _ => OrbbecPlace::Head,
        }
    }
}

impl From<OrbbecPlace> for i64 {
    fn from(value: OrbbecPlace) -> Self {
        match value {
            OrbbecPlace::Head => 0,
            OrbbecPlace::Left => 2,
            OrbbecPlace::Right => 3,
        }
    }
}
