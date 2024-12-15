use clap::Parser;
use razer_rs::{DeviceType, RazerDevice};

mod args;

fn main() {
    println!("Rusty Razer tool!");

    let args = args::Args::parse();
    let mut devices = razer_rs::get_devices();

    for device in &mut devices {
        if !args.device_match(device.get_name()) {
            continue;
        }
        if args.battery() {
            print_battery(device);
        }
        if let Some(low_battery_threshold) = args.low_battery_threshold() {
            device.set_low_battery_threshold(low_battery_threshold);
        }
        if let Some(idle_delay) = args.idle_delay() {
            device.set_idle_time(idle_delay);
        }
        if let Some(dpi) = args.dpi() {
            device.set_dpi(dpi);
        }
        if let Some(poll_rate) = args.poll() {
            device.set_poll_rate(poll_rate);
        }
        if args.syspath() {
            println!("{}", device.get_syspath());
        }
    }

    if args.list() {
        for device in devices {
            if device.get_type() != DeviceType::Unknown {
                print_everything(&device);
            }
        }
    }
}

fn print_battery(device: &RazerDevice) {
    let charge_level = device.get_charge_level();
    let charge_status = device.get_charge_status();
    if charge_level.is_some() || charge_status.is_some() {
        println!("{} Battery:", device.get_name());
        if let Some(charge_level) = charge_level {
            println!("    charge: {}", charge_level);
        }
        if let Some(charge_status) = charge_status {
            println!("    charging: {}", charge_status);
        }
    }
}

fn print_everything(device: &RazerDevice) {
    println!("{}: ", device.get_name());
    println!("    type: {}", device.get_type());

    // Print device type specific settings
    match device.get_type() {
        DeviceType::Mouse => {
            println!("    Mouse settings");
            if let Some(dpi) = device.get_dpi() {
                println!("        DPI: {}", dpi);
            }
            if let Some(max_dpi) = device.get_max_dpi() {
                println!("        max DPI: {}", max_dpi);
            }
            if let Some(dpi_stages) = device.get_dpi_stages() {
                println!("        DPI stages: {:?}", dpi_stages);
            }
            if let Some(poll_rate) = device.get_poll_rate() {
                println!("        polling rate: {}", poll_rate);
            }
        }
        _ => todo!(),
    }

    // Print Battery section
    let charge_level = device.get_charge_level();
    let charge_status = device.get_charge_status();
    if charge_level.is_some() || charge_status.is_some() {
        println!("    Battery:");
        if let Some(charge_level) = charge_level {
            println!("        charge: {}", charge_level);
        }
        if let Some(charge_status) = charge_status {
            println!("        charging: {}", charge_status);
        }
        if let Some(low_battery_threshold) = device.get_low_battery_threshold() {
            println!("        low battery threshold: {}", low_battery_threshold);
        }
        if let Some(idle_time) = device.get_idle_time() {
            println!("        idle time: {}", idle_time);
        }
    }

    // TODO: rgb stuff
}
