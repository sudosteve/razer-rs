use std::i32;
use udev::Enumerator;

fn main() {
    println!("Rusty Razer tool!");
    list_devices();
}

// TODO add filters struct

fn list_devices() {
    let mut enumerator = Enumerator::new().unwrap();
    let razer_vendor_id = 0x1532;

    enumerator.match_subsystem("hid").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        let sysname = device.sysname().to_str().unwrap();
        let (vendor_id, _) = get_vendor_id_device_id(sysname);

        if vendor_id == razer_vendor_id {
            if let Some(device_type) = device.attribute_value("device_type") {
                println!("{}: ", device_type.to_str().unwrap());
                if let Some(charge_level_str) = device.attribute_value("charge_level") {
                    let charge_level_str = charge_level_str.to_str().unwrap();
                    let charge_level = charge_level_str.parse::<i32>().unwrap() * 100 / 255;
                    println!("    charge: {}", charge_level);
                }
                if let Some(charge_status_str) = device.attribute_value("charge_status") {
                    let charge_status_str = charge_status_str.to_str().unwrap();
                    let charge_status = charge_status_str.eq("1");
                    println!("    charging: {}", charge_status);
                }
            }
        }
    }
}

fn get_vendor_id_device_id(sysname: &str) -> (i32, i32) {
    let mut parts = sysname.split(":");
    parts.next();
    let vendor_id = i32::from_str_radix(parts.next().unwrap(), 16).unwrap();
    let device_id_str = parts.next().unwrap().split(".").next().unwrap();
    let device_id = i32::from_str_radix(device_id_str, 16).unwrap();
    (vendor_id, device_id)
}
