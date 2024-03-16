use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List all device info
    #[arg(short, long)]
    list: bool,

    /// Specify device
    #[arg(short, long)]
    device: Option<String>,

    /// Change dpi
    #[arg(long)]
    dpi: Option<u16>,

    /// Change polling rate
    #[arg(long)]
    poll: Option<u16>,
}

fn main() {
    println!("Rusty Razer tool!");

    // TODO: use args
    let _args = Args::parse();
    let devices = razer_rs::get_devices();

    for mut device in devices {
        println!("{}: ", device.get_name());
        println!("    type: {}", device.get_type());
        if let Some(dpi) = device.get_dpi() {
            println!("    DPI: {}", dpi);
        }
        if let Some(max_dpi) = device.get_max_dpi() {
            println!("    max DPI: {}", max_dpi);
        }
        if let Some(dpi_stages) = device.get_dpi_stages() {
            println!("    DPI stages: {:?}", dpi_stages);
        }
        if let Some(charge_level) = device.get_charge_level() {
            println!("    charge: {}", charge_level);
        }
        if let Some(charge_status) = device.get_charge_status() {
            println!("    charging: {}", charge_status);
        }
    }
}

// TODO add filters struct

// fn list_devices() {
// }
