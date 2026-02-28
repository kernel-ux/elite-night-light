use zbus::interface;

pub struct NightLight {
    enabled: bool,
    level: u8,
}

impl NightLight {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: 2, // Default to Warm
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_level(&self) -> u8 {
        self.level
    }
}

#[interface(name = "io.github.kernel_ux.CosmicComp.NightLight")]
impl NightLight {
    #[zbus(name = "Enabled")]
    fn enabled(&self) -> bool {
        self.enabled
    }

    #[zbus(name = "Level")]
    fn level(&self) -> u8 {
        self.level
    }

    #[zbus(name = "SetEnabled")]
    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[zbus(name = "SetLevel")]
    fn set_level(&mut self, level: u8) {
        self.level = level;
    }
}
