use serde::Serialize;

pub const TABLE_SETTING: &str = "setting";

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Setting {
    pub id: u32,               // 主键id
    pub device_type: i64,      // 设备类型 1:U2D2  2:奥比中光335相机
    pub device_place: i64,     // 位置描述
    pub device_name: String,   // 设备取名字
    pub device_serial: String, // 设备序列号
    pub created_at: i64,       // 创建时间
}
