use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::u16;
use std::vec::Vec;
use udev::{Device, Enumerator};

const RAZER_VENDOR_ID: u16 = 0x1532;

pub struct RazerDevice {
    device: Device,
    name: String,
}

impl RazerDevice {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_dpi(&self) -> Option<u16> {
        self.device.attribute_value("dpi").map(|s| {
            s.to_str().unwrap().parse::<u16>().unwrap()
        })
    }

    pub fn set_dpi(&mut self, dpi: u16) {
        let path = Path::new(self.device.syspath()).join("dpi");
        let bytes = dpi.to_be_bytes();
        if let Ok(mut f) = File::options().write(true).open(path) {
            f.write_all(&bytes).expect("Failed to write to dpi file");
        }
    }

    pub fn set_dpi_xy(&mut self, dpi_x: u16, dpi_y: u16) {
        let path = Path::new(self.device.syspath()).join("dpi");
        let mut byte_vec = dpi_x.to_be_bytes().to_vec();
        byte_vec.append(&mut dpi_y.to_be_bytes().to_vec());
        if let Ok(mut f) = File::options().write(true).open(path) {
            f.write_all(&byte_vec).expect("Failed to write to dpi file");
        }
    }

    pub fn get_poll_rate(&self) -> Option<u16> {
        self.device.attribute_value("poll_rate").map(|s| {
            s.to_str().unwrap().parse::<u16>().unwrap()
        })
    }

    pub fn set_poll_rate(&mut self, dpi: u16) {
        let _ = self.device.set_attribute_value("poll_rate", dpi.to_string());
    }

    pub fn get_charge_level(&self) -> Option<u16> {
        self.device.attribute_value("charge_level").map(|s| {
            s.to_str().unwrap().parse::<u16>().unwrap() * 100 / 255
        })
    }

    pub fn get_charge_status(&self) -> Option<bool> {
        self.device.attribute_value("charge_status").map(|s| {
            s.to_str().unwrap().eq("1")
        })
    }

    pub fn get_charge_low_threshold(&self) -> Option<u16> {
        self.device.attribute_value("charge_low_threshold").map(|s| {
            s.to_str().unwrap().parse::<u16>().unwrap() * 100 / 255
        })
    }

    pub fn get_idle_time(&self) -> Option<u16> {
        self.device.attribute_value("charge_status").map(|s| {
            s.to_str().unwrap().parse::<u16>().unwrap()
        })
    }

    pub fn set_idle_time(&mut self, idle_time: u16) {
        let _ = self.device.set_attribute_value("device_idle_time", idle_time.to_string());
    }
}

pub fn get_devices() -> Vec<RazerDevice> {
    let mut result: Vec<RazerDevice> = Vec::new();
    let mut enumerator = Enumerator::new().unwrap();

    enumerator.match_subsystem("hid").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        let sysname = device.sysname().to_str().unwrap();
        let (vendor_id, _) = get_vendor_id_device_id(sysname);

        if vendor_id == RAZER_VENDOR_ID {
            if let Some(device_type) = device.attribute_value("device_type") {
                let name = String::from(device_type.to_str().unwrap());
                result.push(RazerDevice{
                    device: device,
                    name: name,
                });
            }
        }
    }
    result
}

fn get_vendor_id_device_id(sysname: &str) -> (u16, u16) {
    let mut parts = sysname.split(":");
    parts.next();
    let vendor_id = u16::from_str_radix(parts.next().unwrap(), 16).unwrap();
    let device_id_str = parts.next().unwrap().split(".").next().unwrap();
    let device_id = u16::from_str_radix(device_id_str, 16).unwrap();
    (vendor_id, device_id)
}
