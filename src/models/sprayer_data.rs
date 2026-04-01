use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SprayerData {
    pub current_pressure: f32,
    pub target_pressure: f32,
    pub speed: f32,
    pub boom_locked: bool,
}

impl Default for SprayerData {
    fn default() -> Self {
        Self {
            current_pressure: 0.0,
            target_pressure: 0.0,
            speed: 0.0,
            boom_locked: false,
        }
    }
}

impl SprayerData {
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() < 13 {
            anyhow::bail!("Invalid byte array length for SprayerData: expected 13, got {}", bytes.len());
        }

        // Based on the project description: 13-byte binary packet
        // target pressure (4 bytes), current pressure (4 bytes), speed (4 bytes), boom lock status (1 byte)
        
        let target_pressure = f32::from_le_bytes(bytes[0..4].try_into()?);
        let current_pressure = f32::from_le_bytes(bytes[4..8].try_into()?);
        let speed = f32::from_le_bytes(bytes[8..12].try_into()?);
        let boom_locked = bytes[12] != 0;

        Ok(Self {
            current_pressure,
            target_pressure,
            speed,
            boom_locked,
        })
    }
}
