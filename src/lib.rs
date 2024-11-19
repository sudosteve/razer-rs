use std::ffi::OsStr;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;
use std::{fmt};

mod devices;

const RAZER_VENDOR_ID: u16 = 0x1532;

#[derive(Copy, Clone, Default)]
pub enum DeviceType {
    #[default]
    Unknown,
    Mouse,
    Keyboard,
    Headphones,
    Mousepad,
}

#[derive(Clone)]
pub struct DeviceCapabilities {
    pub name: &'static str,
    pub device_type: DeviceType,
    pub dpi: bool,
    pub dpi_use_xy: bool,
    pub max_dpi: Option<u16>,
    pub dpi_stages: bool,
    pub poll_rate: bool,
    pub battery: bool, // TODO: Might need to add low_threshold and idle_delay
}

pub struct RazerDevice {
    name: String,
    device_capabilities: Option<DeviceCapabilities>,
    udev_device: udev::Device,
}

impl RazerDevice {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_syspath(&self) -> &str {
        self.udev_device.syspath().to_str().unwrap()
    }

    pub fn get_type(&self) -> DeviceType {
        self.device_capabilities
            .as_ref()
            .map(|d| d.device_type)
            .unwrap_or_default()
    }

    pub fn get_dpi(&self) -> Option<Dpi> {
        if self.device_capabilities.as_ref().is_some_and(|d| !d.dpi) {
            return None;
        }
        let dpi_str: Option<&OsStr> = self.udev_device.attribute_value("dpi");
        if self
            .device_capabilities
            .as_ref()
            .is_some_and(|d| d.dpi_use_xy)
        {
            dpi_str.and_then(split_xy).map(|(x, y)| Dpi::XY(x, y))
        } else {
            dpi_str.map(|s| Dpi::Single(s.to_str().unwrap().parse::<u16>().unwrap()))
        }
    }

    pub fn set_dpi(&mut self, dpi: u16) {
        if self.device_capabilities.as_ref().is_some_and(|d| !d.dpi) {
            return;
        }
        self.set_raw_attribute_value("dpi", &dpi.to_be_bytes());
    }

    pub fn set_dpi_xy(&mut self, dpi_x: u16, dpi_y: u16) {
        let mut byte_vec = dpi_x.to_be_bytes().to_vec();
        byte_vec.append(&mut dpi_y.to_be_bytes().to_vec());
        self.set_raw_attribute_value("dpi", &byte_vec);
    }

    pub fn get_max_dpi(&self) -> Option<u16> {
        self.device_capabilities.as_ref().and_then(|d| d.max_dpi)
    }

    pub fn get_dpi_stages(&self) -> Option<(u8, Vec<(u16, u16)>)> {
        if self
            .device_capabilities
            .as_ref()
            .is_some_and(|d| !d.dpi_stages)
        {
            return None;
        }
        self.get_raw_attribute_value("dpi_stages").map(|v| {
            let mut stages: Vec<(u16, u16)> = Vec::with_capacity((v.len() - 1) / 4);
            let active_stage = v[0];
            let mut bytes = &v[1..];
            while bytes.len() >= 4 {
                let stage_x =
                    u16::from_be_bytes(bytes[0..2].try_into().expect("This is impossible"));
                let stage_y =
                    u16::from_be_bytes(bytes[2..4].try_into().expect("This is impossible"));
                stages.push((stage_x, stage_y));
                bytes = &bytes[4..]
            }
            (active_stage, stages)
        })
    }

    pub fn get_poll_rate(&self) -> Option<u16> {
        if self
            .device_capabilities
            .as_ref()
            .is_some_and(|d| !d.poll_rate)
        {
            return None;
        }
        self.udev_device
            .attribute_value("poll_rate")
            .map(|s| s.to_str().unwrap().parse::<u16>().unwrap())
    }

    pub fn set_poll_rate(&mut self, dpi: u16) {
        if self
            .device_capabilities
            .as_ref()
            .is_some_and(|d| !d.poll_rate)
        {
            return;
        }
        let _ = self
            .udev_device
            .set_attribute_value("poll_rate", dpi.to_string());
    }

    pub fn get_charge_level(&self) -> Option<u16> {
        self.udev_device
            .attribute_value("charge_level")
            .map(|s| s.to_str().unwrap().parse::<u16>().unwrap() * 100 / 255)
    }

    pub fn get_charge_status(&self) -> Option<bool> {
        self.udev_device
            .attribute_value("charge_status")
            .map(|s| s.to_str().unwrap().eq("1"))
    }

    pub fn get_low_battery_threshold(&self) -> Option<u16> {
        self.udev_device
            .attribute_value("charge_low_threshold")
            .map(|s| s.to_str().unwrap().parse::<u16>().unwrap() * 100 / 255)
    }

    pub fn set_low_battery_threshold(&mut self, low_battery_threshold: u16) {
        let _ = self
            .udev_device
            .set_attribute_value("charge_low_threshold", low_battery_threshold.to_string());
    }

    pub fn get_idle_time(&self) -> Option<u16> {
        self.udev_device
            .attribute_value("charge_status")
            .map(|s| s.to_str().unwrap().parse::<u16>().unwrap())
    }

    pub fn set_idle_time(&mut self, idle_time: u16) {
        let _ = self
            .udev_device
            .set_attribute_value("device_idle_time", idle_time.to_string());
    }

    pub fn get_raw_attribute_value<P: AsRef<Path>>(&self, attr: P) -> Option<Vec<u8>> {
        let mut result = Vec::new();
        let path = Path::new(self.udev_device.syspath()).join(attr);
        let mut file = File::options().read(true).open(path).ok()?;
        let bytes_read = file.read_to_end(&mut result).ok()?;
        assert!(bytes_read == result.len());
        Some(result)
    }

    fn set_raw_attribute_value<P: AsRef<Path>>(&self, attr: P, value: &[u8]) {
        let path = Path::new(self.udev_device.syspath()).join(attr);
        if let Ok(mut f) = File::options().write(true).open(path) {
            f.write_all(value).expect("Failed to write to file");
        }
    }

    // pub fn get_lighting_options(&self) -> (BTreeSet<String>, BTreeSet<String>) {
    //     let mut regions = BTreeSet::new();
    //     let mut effects = BTreeSet::new();
    //     for attribute in self.udev_device.attributes() {
    //         let mut attribute = attribute.name().to_str().unwrap();
    //         if let Some(i) = attribute.find("matrix_effect") {
    //             let mut region = "";
    //             if i > 0  {
    //                 (region, attribute) = attribute.split_at(i);
    //                 let mut region = String::from(region);
    //                 region.pop();
    //                 println!("{}", region);
    //                 regions.insert(region);
    //             }
    //             let (_, effect) = attribute.split_at("matrix_effect_".len());
    //             println!("{}", effect);
    //             effects.insert(String::from(effect));
    //         }
    //     }
    //     (regions, effects)
    // }
}

pub fn get_devices() -> Vec<RazerDevice> {
    let mut result: Vec<RazerDevice> = Vec::new();
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("hid").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        let sysname = device.sysname().to_str().unwrap();
        let (vendor_id, device_id) = get_vendor_id_device_id(sysname);

        if vendor_id == RAZER_VENDOR_ID {
            if let Some(device_name) = device.attribute_value("device_type") {
                let name = String::from(device_name.to_str().unwrap());
                let device_capabilities = devices::get_device_capabilities(device_id);
                if device_capabilities.is_some() {
                    assert!(device_capabilities.as_ref().unwrap().name == name);
                }
                result.push(RazerDevice {
                    name,
                    device_capabilities,
                    udev_device: device,
                });
            }
        }
    }
    result
}

fn get_vendor_id_device_id(sysname: &str) -> (u16, u16) {
    let mut parts = sysname.split(':');
    parts.next();
    let vendor_id = u16::from_str_radix(parts.next().unwrap(), 16).unwrap();
    let device_id_str = parts.next().unwrap().split('.').next().unwrap();
    let device_id = u16::from_str_radix(device_id_str, 16).unwrap();
    (vendor_id, device_id)
}

fn split_xy(xy_str: &OsStr) -> Option<(u16, u16)> {
    let mut split = xy_str.to_str().unwrap().split(':');
    match (split.next(), split.next()) {
        (Some(x), Some(y)) => Some((x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap())),
        _ => None,
    }
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceType::Unknown => write!(f, "unknown"),
            DeviceType::Mouse => write!(f, "mouse"),
            DeviceType::Keyboard => write!(f, "keyboard"),
            DeviceType::Headphones => write!(f, "headphones"),
            DeviceType::Mousepad => write!(f, "mousepad"),
        }
    }
}

#[derive(Clone)]
pub enum Dpi {
    Single(u16),
    XY(u16, u16),
}

impl fmt::Display for Dpi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dpi::Single(x) => write!(f, "{}", x),
            Dpi::XY(x, y) => write!(f, "({}, {})", x, y),
        }
    }
}

// Lighting structs
pub enum Region {
    Blank,
    Scroll,
    Logo,
    Left,
    Right,
    Backlight,
}

pub enum Effect {
    None,
    Custom,
    Static, // TODO add RGB
    Wave,
    Spectrum,
    Reactive,
    Breath, // TODO add optional RGB
}
