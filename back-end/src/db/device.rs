use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub last_heartbeat: u32,
    pub mac_address: String,
}
