pub const STATUS_HEADER: [u8; 5] = [0x80, 0x81, 0x70, 0x70, 0x07];
pub const SETTINGS_HEADER_PREFIX: [u8; 5] = [0x80, 0x81, 0x71, 0x70, 0x0A];
pub const BUTTON_HEADER_PREFIX: [u8; 5] = [0x80, 0x81, 0x71, 0x71, 0x01];

pub const STATUS_PACKET_LEN: usize = 13;
pub const SETTINGS_PACKET_LEN: usize = 16;
pub const BUTTON_PACKET_LEN: usize = 7;

pub const DEFAULT_MULTIPLIER: f32 = 100.0;
pub const LITRES_PER_HA_MULTIPLIER: f32 = 10.0;

pub const DEFAULT_STATUS_PORT: u16 = 1111;
pub const DEFAULT_COMMAND_PORT: u16 = 8888;
