use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Light {
    pub color: String,
    pub brightness: u8,
}
