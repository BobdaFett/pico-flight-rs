use core::sync::atomic::{AtomicBool, Ordering};
use defmt::info;
use embassy_usb::Handler;

pub struct DeviceHandler {
    configured: AtomicBool
}

impl DeviceHandler {
    pub fn new() -> Self {
        Self {
            configured: AtomicBool::new(false)
        }
    }
}

impl Handler for DeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device is enabled.");
        } else {
            info!("Device is disabled.");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Device is reset.");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Device is addressed, address set to {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
           info!("Device is now configured, it can draw full power");
        } else {
            info!("Device is unconfigured, VBUS limit is 100mA");
        }
    }
}
