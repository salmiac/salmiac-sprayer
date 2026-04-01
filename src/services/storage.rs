use crate::models::sprayer_settings::SprayerSettings;
use anyhow::Result;

pub struct SprayerSettingsStorage;

impl SprayerSettingsStorage {
    const APP_NAME: &'static str = "salmiac-sprayer";

    pub fn save_settings(settings: &SprayerSettings) -> Result<()> {
        confy::store(Self::APP_NAME, None, settings)?;
        Ok(())
    }

    pub fn load_settings() -> Result<SprayerSettings> {
        let settings: SprayerSettings = confy::load(Self::APP_NAME, None)?;
        Ok(settings)
    }
}
