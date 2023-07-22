use std::i32;
use std::vec::Vec;
use udev::{Device, Enumerator};

const RAZER_VENDOR_ID: i32 = 0x1532;

pub struct RazerDevice {
    device: Device,
    name: String,
}

impl RazerDevice {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_charge_level(&self) -> Option<i32> {
        self.device.attribute_value("charge_level").map(|s| {
            s.to_str().unwrap().parse::<i32>().unwrap() * 100 / 255
        })
    }

    pub fn get_charge_status(&self) -> Option<bool> {
        self.device.attribute_value("charge_status").map(|s| {
            s.to_str().unwrap().eq("1")
        })
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

fn get_vendor_id_device_id(sysname: &str) -> (i32, i32) {
    let mut parts = sysname.split(":");
    parts.next();
    let vendor_id = i32::from_str_radix(parts.next().unwrap(), 16).unwrap();
    let device_id_str = parts.next().unwrap().split(".").next().unwrap();
    let device_id = i32::from_str_radix(device_id_str, 16).unwrap();
    (vendor_id, device_id)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
