use std::net::UdpSocket;
use tokio::sync::broadcast;
use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::SprayerSettings;

#[derive(Clone)]
pub struct ControllerService {
    tx: broadcast::Sender<SprayerData>,
}

impl ControllerService {
    pub fn new() -> (Self, broadcast::Receiver<SprayerData>) {
        let (tx, rx) = broadcast::channel(100);
        (Self { tx }, rx)
    }

    pub async fn start_udp_receiver(&self, port: u16) -> anyhow::Result<()> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port))?;
        socket.set_nonblocking(true)?;
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match socket.recv_from(&mut buf) {
                    Ok((len, _)) => {
                        if let Ok(data) = Self::parse_sprayer_data(&buf[..len]) {
                            let _ = tx.send(data);
                        }
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(())
    }

    fn parse_sprayer_data(data: &[u8]) -> anyhow::Result<SprayerData> {
        if data.len() != 13 {
            anyhow::bail!("Invalid data length");
        }

        if data[0..5] != [0x80, 0x81, 0x70, 0x70, 0x07] {
            anyhow::bail!("Invalid header");
        }

        let target_pressure = u16::from_le_bytes([data[5], data[6]]) as f32 / 100.0;
        let current_pressure = u16::from_le_bytes([data[7], data[8]]) as f32 / 100.0;
        let speed = u16::from_le_bytes([data[9], data[10]]) as f32 / 100.0;
        let boom_locked = data[11] == 1;

        let calculated_crc = data[2..12].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        if calculated_crc != data[12] {
            anyhow::bail!("CRC mismatch");
        }

        Ok(SprayerData {
            current_pressure,
            target_pressure,
            speed,
            boom_locked,
        })
    }

    pub fn send_settings(&self, target_ip: &str, port: u16, settings: &SprayerSettings) -> anyhow::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        let mut data = vec![0x80, 0x81, 0x71, 0x70, 0x0A];
        
        let nozzle_size_int = (settings.nozzle_size.size_value * 100.0).round() as u8;
        let nozzle_spacing_int = (settings.nozzle_spacing * 100.0).round() as u8;
        let litres_per_ha_int = (settings.litres_per_ha * 100.0).round() as u16;
        let min_pressure_int = (settings.min_pressure * 100.0).round() as u16;
        let max_pressure_int = (settings.max_pressure * 100.0).round() as u16;
        let nominal_pressure_int = (settings.nominal_pressure * 100.0).round() as u16;

        data.push(nozzle_size_int);
        data.push(nozzle_spacing_int);
        data.extend_from_slice(&litres_per_ha_int.to_le_bytes());
        data.extend_from_slice(&min_pressure_int.to_le_bytes());
        data.extend_from_slice(&max_pressure_int.to_le_bytes());
        data.extend_from_slice(&nominal_pressure_int.to_le_bytes());

        let crc = data[2..].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        data.push(crc);

        socket.send_to(&data, format!("{}:{}", target_ip, port))?;
        Ok(())
    }

    pub fn send_button_state(&self, target_ip: &str, port: u16, activated: bool, constant_pressure: bool) -> anyhow::Result<()> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;

        let mut data = vec![0x80, 0x81, 0x71, 0x71, 0x01];
        let mut button_states = 0u8;
        if activated { button_states |= 0x01; }
        if constant_pressure { button_states |= 0x02; }
        data.push(button_states);

        let crc = data[2..].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        data.push(crc);

        socket.send_to(&data, format!("{}:{}", target_ip, port))?;
        Ok(())
    }
}
