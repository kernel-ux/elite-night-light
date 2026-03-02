use zbus::interface;
use std::sync::{Arc, LazyLock};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub static NIGHT_LIGHT_STATE: LazyLock<Arc<NightLightState>> = LazyLock::new(|| {
    Arc::new(NightLightState::new())
});

#[derive(Debug)]
pub struct NightLightState {
    pub enabled: AtomicBool,
    pub level: AtomicU8,
}

impl NightLightState {
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            level: AtomicU8::new(0),
        }
    }
}

pub struct NightLightInterface;

#[interface(name = "io.github.kernel_ux.EliteNightLight")]
impl NightLightInterface {
    #[zbus(property)]
    fn enabled(&self) -> bool {
        NIGHT_LIGHT_STATE.enabled.load(Ordering::Relaxed)
    }

    #[zbus(property)]
    fn level(&self) -> u8 {
        NIGHT_LIGHT_STATE.level.load(Ordering::Relaxed)
    }

    #[zbus(name = "SetEnabled")]
    fn set_enabled(&self, enabled: bool) {
        NIGHT_LIGHT_STATE.enabled.store(enabled, Ordering::Relaxed);
    }

    #[zbus(name = "SetLevel")]
    fn set_level(&self, level: u8) {
        NIGHT_LIGHT_STATE.level.store(level, Ordering::Relaxed);
    }
}
