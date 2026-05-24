use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Nozzle {
    pub color_name: String,
    pub color_code: [u8; 3], // RGB
    pub number: String,
    pub size_value: f32,
}

pub fn get_nozzle_types() -> Vec<Nozzle> {
    vec![
        Nozzle {
            color_name: "Orange".to_string(),
            color_code: [255, 165, 0],
            number: "01".to_string(),
            size_value: 0.10,
        },
        Nozzle {
            color_name: "Green".to_string(),
            color_code: [0, 128, 0],
            number: "015".to_string(),
            size_value: 0.15,
        },
        Nozzle {
            color_name: "Yellow".to_string(),
            color_code: [255, 255, 0],
            number: "02".to_string(),
            size_value: 0.20,
        },
        Nozzle {
            color_name: "Lilac".to_string(),
            color_code: [255, 192, 203],
            number: "025".to_string(),
            size_value: 0.25,
        },
        Nozzle {
            color_name: "Blue".to_string(),
            color_code: [0, 0, 255],
            number: "03".to_string(),
            size_value: 0.30,
        },
        Nozzle {
            color_name: "Dark Red".to_string(),
            color_code: [139, 0, 0],
            number: "035".to_string(),
            size_value: 0.35,
        },
        Nozzle {
            color_name: "Red".to_string(),
            color_code: [255, 0, 0],
            number: "04".to_string(),
            size_value: 0.40,
        },
        Nozzle {
            color_name: "Brown".to_string(),
            color_code: [165, 42, 42],
            number: "05".to_string(),
            size_value: 0.50,
        },
        Nozzle {
            color_name: "Gray".to_string(),
            color_code: [128, 128, 128],
            number: "06".to_string(),
            size_value: 0.60,
        },
        Nozzle {
            color_name: "White".to_string(),
            color_code: [255, 255, 255],
            number: "08".to_string(),
            size_value: 0.80,
        },
        Nozzle {
            color_name: "Light Blue".to_string(),
            color_code: [173, 216, 230],
            number: "1".to_string(),
            size_value: 1.00,
        },
        Nozzle {
            color_name: "Light Green".to_string(),
            color_code: [144, 238, 144],
            number: "15".to_string(),
            size_value: 1.50,
        },
        Nozzle {
            color_name: "Black".to_string(),
            color_code: [0, 0, 0],
            number: "2".to_string(),
            size_value: 2.00,
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprayerSettings {
    pub nozzle_size: Nozzle,
    pub litres_per_ha: f32,
    pub min_pressure: f32,
    pub max_pressure: f32,
    pub nominal_pressure: f32,
    pub nozzle_spacing: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub pressure_alert_threshold: f32,
    pub target_ip: String,
}

impl Default for SprayerSettings {
    fn default() -> Self {
        let nozzles = get_nozzle_types();
        let default_nozzle = nozzles
            .iter()
            .find(|n| n.number == "025")
            .cloned()
            .unwrap_or_else(|| nozzles[0].clone());

        Self {
            nozzle_size: default_nozzle,
            litres_per_ha: 200.0,
            min_pressure: 1.0,
            max_pressure: 6.0,
            nominal_pressure: 3.0,
            nozzle_spacing: 0.5,
            min_speed: 0.0,
            max_speed: 0.0,
            pressure_alert_threshold: 0.5,
            target_ip: "255.255.255.255".to_string(),
        }
    }
}

impl SprayerSettings {
    pub fn to_bytes(&self, activated: bool, constant_pressure: bool) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(16);

        // Custom 16-byte binary format (tentative based on Dart and GEMINI.md)
        bytes.push(if activated { 1 } else { 0 }); // 1
        bytes.extend_from_slice(&self.nominal_pressure.to_le_bytes()); // 5
        bytes.push(if constant_pressure { 1 } else { 0 }); // 6

        // Nozzle number as 3 ASCII digits (padding to 3)
        let mut nozzle_num = self.nozzle_size.number.clone();
        while nozzle_num.len() < 3 {
            nozzle_num.insert(0, '0');
        }
        bytes.extend_from_slice(nozzle_num.as_bytes()); // 9

        // Pad to 16 bytes
        while bytes.len() < 16 {
            bytes.push(0);
        }

        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_target_ip() {
        let settings = SprayerSettings::default();
        assert_eq!(settings.target_ip, "255.255.255.255");
    }

    #[test]
    fn test_ipv4_parsing_behavior() {
        assert!("192.168.1.100".parse::<std::net::Ipv4Addr>().is_ok());
        assert!("255.255.255.255".parse::<std::net::Ipv4Addr>().is_ok());
        assert!("256.256.256.256".parse::<std::net::Ipv4Addr>().is_err());
        assert!("not_an_ip".parse::<std::net::Ipv4Addr>().is_err());
    }
}
