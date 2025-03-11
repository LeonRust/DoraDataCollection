use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArmState {
    pub joint_status: JointStatus,
}

#[derive(Debug, Deserialize)]
pub struct JointStatus {
    pub joint_position: Vec<f64>,
}
