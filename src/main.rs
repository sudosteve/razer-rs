fn main() {
    println!("Rusty Razer tool!");

    let devices = razer_rs::get_devices();

    for mut device in devices {
        println!("{}: ", device.get_name());

        if let Some(charge_level) = device.get_charge_level() {
            println!("    charge: {}", charge_level);
        }
        if let Some(charge_status) = device.get_charge_status() {
            println!("    charge: {}", charge_status);
        }
    }
}

// TODO add filters struct

// fn list_devices() {
// }
