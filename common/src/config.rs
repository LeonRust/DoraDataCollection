// Daemon Server
pub const DAEMON_IP: &str = "DAEMON_IP"; // Daemon Server IP
pub const DAEMON_TCP_PORT: &str = "DAEMON_TCP_PORT"; // Daemon Server TCP Port
pub const DAEMON_HTTP_PORT: &str = "DAEMON_HTTP_PORT"; // Daemon Server HTTP Port

pub const DATABASE_PATH: &str = "DATABASE_PATH"; // 数据集路径
pub const DATASET_PATH: &str = "DATASET_PATH"; // 数据集路径
pub const CACHE_PATH: &str = "CACHE_PATH"; // 缓存路径

// The clock interval, default 100ms
pub const CLOCK_INTERVAL: &str = "CLOCK_INTERVAL"; // 时钟间隔，单位毫秒
pub const DEFAULT_CLOCK_INTERVAL: u64 = 100; // 默认时钟间隔，单位毫秒

pub const ARM_IP: &str = "ARM_IP"; // 机械臂ip
pub const ARM_DIRECTION: &str = "ARM_DIRECTION"; // 机械臂方向 left / right

pub const CAMERA_ID: &str = "CAMERA_ID"; // 摄像头id OpenCV使用
pub const CAMERA_NUMBER: &str = "CAMERA_NUMBER"; // 摄像头编号 保存目录使用

// usb设备检测
pub const USB_U2D2: (&str, &str) = ("0403:6014", "ttyUSB"); // u2d2
pub const USB_ORBBEC: (&str, &str) = ("2bc5:0800", "video"); // 奥比中光
