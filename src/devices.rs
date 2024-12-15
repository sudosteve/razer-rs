use crate::{DeviceCapabilities, DeviceType};
use phf::phf_map;

pub fn get_device_capabilities(device_id: u16) -> Option<DeviceCapabilities> {
    DEVICE_CAPABILITY_MAP.get(&device_id).cloned()
}

static DEVICE_CAPABILITY_MAP: phf::Map<u16, DeviceCapabilities> = phf_map! {
  0x007Au16 => DeviceCapabilities {
    name: "Razer Viper Ultimate (Wired)",
    device_type: DeviceType::Mouse,
    dpi: true,
    dpi_use_xy: true,
    max_dpi: Some(20000),
    dpi_stages: true,
    poll_rate: true,
    battery: true,
  },
  0x007Bu16 => DeviceCapabilities {
    name: "Razer Viper Ultimate (Wireless)",
    device_type: DeviceType::Mouse,
    dpi: true,
    dpi_use_xy: true,
    max_dpi: Some(20000),
    dpi_stages: true,
    poll_rate: true,
    battery: true,
  },
  0x00B6u16 => DeviceCapabilities {
    name: "Razer DeathAdder V3 Pro (Wired)",
    device_type: DeviceType::Mouse,
    dpi: true,
    dpi_use_xy: true,
    max_dpi: Some(35000),
    dpi_stages: true,
    poll_rate: true,
    battery: true,
  },
  0x00B7u16 => DeviceCapabilities {
    name: "Razer DeathAdder V3 Pro (Wireless)",
    device_type: DeviceType::Mouse,
    dpi: true,
    dpi_use_xy: true,
    max_dpi: Some(35000),
    dpi_stages: true,
    poll_rate: true,
    battery: true,
  },
};
