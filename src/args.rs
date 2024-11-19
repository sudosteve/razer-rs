use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Specify device
    #[arg(short, long)]
    device: Option<String>,

    /// Print battery info
    #[arg(short, long)]
    battery: bool,

    /// Set low battery blink threshold
    #[arg(long)]
    low_battery_threshold: Option<u16>,

    /// Set idle delay
    #[arg(short, long)]
    idle_delay: Option<u16>,

    /// Change dpi
    #[arg(long)]
    dpi: Option<u16>,

    /// Change polling rate
    #[arg(long)]
    poll: Option<u16>,

    /// List all devices and their settings
    #[arg(short, long)]
    list: bool,

    /// Print syspath
    #[arg(short, long)]
    syspath: bool
}

impl Args {
    pub fn device_match(&self, device_name: &str) -> bool {
        self.device
            .as_ref()
            .is_none_or(|device_arg| device_arg == device_name)
    }

    pub fn battery(&self) -> bool {
        self.battery
    }

    pub fn low_battery_threshold(&self) -> Option<u16> {
        self.low_battery_threshold
    }

    pub fn idle_delay(&self) -> Option<u16> {
        self.idle_delay
    }

    pub fn dpi(&self) -> Option<u16> {
        self.dpi
    }

    pub fn poll(&self) -> Option<u16> {
        self.poll
    }

    pub fn list(&self) -> bool {
        self.list
    }

    pub fn syspath(&self) -> bool {
        self.syspath
    }
}
