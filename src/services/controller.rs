use crate::models::sprayer_data::SprayerData;
use crate::models::sprayer_settings::SprayerSettings;
use crate::protocol::*;
use tokio::net::UdpSocket;
use tokio::sync::broadcast;

#[cfg(target_os = "android")]
fn acquire_android_locks() -> anyhow::Result<()> {
    use jni::objects::JObject;
    use jni::objects::JValue;

    log::info!("Attempting to acquire Android Network Locks (Multicast + Wifi)...");
    let ctx = ndk_context::android_context();
    let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
    let mut env = vm.attach_current_thread()?;
    let context_obj = unsafe { JObject::from_raw(ctx.context().cast()) };

    let result: jni::errors::Result<()> = (|| {
        // Get WifiManager
        let wifi_service_name = env.new_string("wifi")?;
        let wifi_manager = env
            .call_method(
                &context_obj,
                "getSystemService",
                "(Ljava/lang/String;)Ljava/lang/Object;",
                &[JValue::Object(&wifi_service_name)],
            )?
            .l()?;

        // 1. Create and Acquire MulticastLock
        let mc_lock_tag = env.new_string("SalmiacMulticastLock")?;
        let multicast_lock = env
            .call_method(
                &wifi_manager,
                "createMulticastLock",
                "(Ljava/lang/String;)Landroid/net/wifi/WifiManager$MulticastLock;",
                &[JValue::Object(&mc_lock_tag)],
            )?
            .l()?;
        env.call_method(&multicast_lock, "acquire", "()V", &[])?;
        log::info!("MulticastLock acquired.");

        // 2. Create and Acquire WifiLock (WIFI_MODE_FULL_HIGH_PERF = 3)
        let wf_lock_tag = env.new_string("SalmiacWifiLock")?;
        let wifi_lock = env
            .call_method(
                &wifi_manager,
                "createWifiLock",
                "(ILjava/lang/String;)Landroid/net/wifi/WifiManager$WifiLock;",
                &[JValue::Int(3), JValue::Object(&wf_lock_tag)],
            )?
            .l()?;
        env.call_method(&wifi_lock, "acquire", "()V", &[])?;
        log::info!("WifiLock acquired.");

        Ok(())
    })();

    match result {
        Ok(_) => {
            log::info!("All Android Network Locks successfully acquired.");
            Ok(())
        }
        Err(e) => {
            log::error!("JNI error during lock acquisition: {:?}", e);
            if env.exception_check()? {
                log::error!("A Java exception occurred!");
                env.exception_describe()?;
                env.exception_clear()?;
            }
            Err(anyhow::anyhow!("Failed to acquire locks: {:?}", e))
        }
    }
}

#[cfg(not(target_os = "android"))]
fn acquire_android_locks() -> anyhow::Result<()> {
    Ok(())
}

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
        // Android requires a MulticastLock to even bind a UDP socket sometimes.
        if let Err(e) = acquire_android_locks() {
            log::warn!("Failed to acquire Android locks: {}", e);
        }

        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).await?;
        log::info!("UDP receiver bound successfully to 0.0.0.0:{}", port);
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        log::debug!("Received {} bytes from {}", len, addr);
                        match SprayerData::from_bytes(&buf[..len]) {
                            Ok(data) => {
                                if let Err(e) = tx.send(data) {
                                    log::debug!("Failed to broadcast sprayer data: {}", e);
                                }
                            }
                            Err(e) => {
                                log::warn!("Protocol parsing error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Network error in UDP receiver: {}. Stopping receiver.", e);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn send_settings(
        &self,
        target_ip: &str,
        port: u16,
        settings: &SprayerSettings,
    ) -> anyhow::Result<()> {
        let std_socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        std_socket.set_broadcast(true)?;
        let data = Self::pack_settings(settings);
        log::debug!(
            "Sending settings to {}:{}. Bytes: {:02X?}",
            target_ip,
            port,
            data
        );
        std_socket.send_to(&data, format!("{}:{}", target_ip, port))?;
        Ok(())
    }

    pub fn send_button_state(
        &self,
        target_ip: &str,
        port: u16,
        activated: bool,
        constant_pressure: bool,
    ) -> anyhow::Result<()> {
        let std_socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        std_socket.set_broadcast(true)?;
        let data = Self::pack_button_state(activated, constant_pressure);
        log::debug!(
            "Sending button state to {}:{}. Bytes: {:02X?}",
            target_ip,
            port,
            data
        );
        std_socket.send_to(&data, format!("{}:{}", target_ip, port))?;
        Ok(())
    }

    fn pack_settings(settings: &SprayerSettings) -> Vec<u8> {
        let mut data = SETTINGS_HEADER_PREFIX.to_vec();

        let nozzle_size_int = (settings.nozzle_size.size_value * DEFAULT_MULTIPLIER).round() as u8;
        let nozzle_spacing_int = (settings.nozzle_spacing * DEFAULT_MULTIPLIER).round() as u8;
        let litres_per_ha_int = (settings.litres_per_ha * LITRES_PER_HA_MULTIPLIER).round() as u16;
        let min_pressure_int = (settings.min_pressure * DEFAULT_MULTIPLIER).round() as u16;
        let max_pressure_int = (settings.max_pressure * DEFAULT_MULTIPLIER).round() as u16;
        let nominal_pressure_int = (settings.nominal_pressure * DEFAULT_MULTIPLIER).round() as u16;

        data.push(nozzle_size_int);
        data.push(nozzle_spacing_int);
        data.extend_from_slice(&litres_per_ha_int.to_le_bytes());
        data.extend_from_slice(&min_pressure_int.to_le_bytes());
        data.extend_from_slice(&max_pressure_int.to_le_bytes());
        data.extend_from_slice(&nominal_pressure_int.to_le_bytes());

        let crc = data[2..].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        data.push(crc);
        data
    }

    fn pack_button_state(activated: bool, constant_pressure: bool) -> Vec<u8> {
        let mut data = BUTTON_HEADER_PREFIX.to_vec();
        let mut button_states = 0u8;
        if activated {
            button_states |= 0x01;
        }
        if constant_pressure {
            button_states |= 0x02;
        }
        data.push(button_states);

        let crc = data[2..].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        data.push(crc);
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_settings() {
        let settings = SprayerSettings::default();
        let data = ControllerService::pack_settings(&settings);

        assert_eq!(data.len(), SETTINGS_PACKET_LEN);
        assert_eq!(data[0..5], SETTINGS_HEADER_PREFIX);

        // Verify CRC
        let expected_crc = data[2..15].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        assert_eq!(data[15], expected_crc);
    }

    #[test]
    fn test_pack_button_state() {
        let data = ControllerService::pack_button_state(true, true);

        assert_eq!(data.len(), BUTTON_PACKET_LEN);
        assert_eq!(data[0..5], BUTTON_HEADER_PREFIX);
        assert_eq!(data[5], 0x03); // Both bits set

        // Verify CRC
        let expected_crc = data[2..6].iter().fold(0u8, |acc, &x| acc.wrapping_add(x));
        assert_eq!(data[6], expected_crc);
    }
}
