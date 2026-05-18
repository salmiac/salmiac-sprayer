use serde::{Deserialize, Serialize};
use crate::protocol::{STATUS_HEADER, STATUS_PACKET_LEN, DEFAULT_MULTIPLIER};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    /// Parses SprayerData from a binary packet.
    /// Format: [Header(5), Target(2), Current(2), Speed(2), Locked(1), CRC(1)]
    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        if bytes.len() != STATUS_PACKET_LEN {
            anyhow::bail!("Invalid data length: expected {}, got {}", STATUS_PACKET_LEN, bytes.len());
        }

        if bytes[0..5] != STATUS_HEADER {
            anyhow::bail!("Invalid header");
        }

        // CRC check (sum of bytes from index 2 to 11)
        let calculated_crc = bytes[2..12].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        if calculated_crc != bytes[12] {
            anyhow::bail!("CRC mismatch: expected {}, got {}", calculated_crc, bytes[12]);
        }

        let target_pressure = u16::from_le_bytes([bytes[5], bytes[6]]) as f32 / DEFAULT_MULTIPLIER;
        let current_pressure = u16::from_le_bytes([bytes[7], bytes[8]]) as f32 / DEFAULT_MULTIPLIER;
        let speed = u16::from_le_bytes([bytes[9], bytes[10]]) as f32 / DEFAULT_MULTIPLIER;
        let boom_locked = bytes[11] == 1;

        Ok(Self {
            current_pressure,
            target_pressure,
            speed,
            boom_locked,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bytes_valid() {
        // Target: 2.50 (250), Current: 2.45 (245), Speed: 8.50 (850), Locked: true (1)
        let mut bytes = vec![0x80, 0x81, 0x70, 0x70, 0x07]; // Header
        bytes.extend_from_slice(&250u16.to_le_bytes());
        bytes.extend_from_slice(&245u16.to_le_bytes());
        bytes.extend_from_slice(&850u16.to_le_bytes());
        bytes.push(1); // Locked
        
        let crc = bytes[2..12].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        bytes.push(crc);

        let result = SprayerData::from_bytes(&bytes).unwrap();
        assert_eq!(result.target_pressure, 2.50);
        assert_eq!(result.current_pressure, 2.45);
        assert_eq!(result.speed, 8.50);
        assert_eq!(result.boom_locked, true);
    }

    #[test]
    fn test_from_bytes_invalid_header() {
        let mut bytes = vec![0x00, 0x00, 0x70, 0x70, 0x07];
        bytes.extend_from_slice(&[0; 8]);
        let result = SprayerData::from_bytes(&bytes);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Invalid header");
    }

    #[test]
    fn test_from_bytes_invalid_len() {
        let bytes = vec![0x80, 0x81, 0x70, 0x70, 0x07];
        let result = SprayerData::from_bytes(&bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid data length"));
    }

    #[test]
    fn test_from_bytes_bad_crc() {
        let mut bytes = vec![0x80, 0x81, 0x70, 0x70, 0x07];
        bytes.extend_from_slice(&[0; 7]);
        bytes.push(0xFF); // Intentionally wrong CRC
        let result = SprayerData::from_bytes(&bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("CRC mismatch"));
    }
}
