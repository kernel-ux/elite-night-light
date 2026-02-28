use zbus::interface;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Debug, Clone)]
pub struct NightLightState {
    pub enabled: bool,
    pub level: u8,
}

impl NightLightState {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: 2,
        }
    }
}

pub struct NightLightInterface {
    pub state: Arc<Mutex<NightLightState>>,
}

#[interface(name = "com.system76.CosmicComp.NightLight")]
impl NightLightInterface {
    #[zbus(name = "Enabled")]
    fn enabled(&self) -> bool {
        self.state.lock().enabled
    }

    #[zbus(name = "Level")]
    fn level(&self) -> u8 {
        self.state.lock().level
    }

    #[zbus(name = "SetEnabled")]
    fn set_enabled(&mut self, enabled: bool) {
        self.state.lock().enabled = enabled;
    }

    #[zbus(name = "SetLevel")]
    fn set_level(&mut self, level: u8) {
        self.state.lock().level = level;
    }
}
