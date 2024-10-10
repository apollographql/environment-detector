use std::ops::Not;

const DMI_BIOS_VENDOR: &str = "/sys/class/dmi/id/bios_vendor";
const DMI_PRODUCT_NAME: &str = "/sys/class/dmi/id/product_name";
const DMI_SYS_VENDOR: &str = "/sys/class/dmi/id/sys_vendor";

/// Represents data obtained from SMBIOS.
#[derive(Debug)]
pub struct Smbios {
    dmi_bios_vendor: Option<String>,
    dmi_product_name: Option<String>,
    dmi_sys_vendor: Option<String>,
}

impl Smbios {
    pub fn new() -> Self {
        // TODO: is this idiomatic?
        if cfg!(target_os = "linux") {
            Self::new_linux()
        } else if cfg!(target_os = "windows") {
            Self::new_windows()
        } else {
            unimplemented!("platform not supported")
        }
    }

    fn new_linux() -> Self {
        let dmi_bios_vendor = read_dmi_data(DMI_BIOS_VENDOR);
        let dmi_product_name = read_dmi_data(DMI_PRODUCT_NAME);
        let dmi_sys_vendor = read_dmi_data(DMI_SYS_VENDOR);

        Self {
            dmi_bios_vendor,
            dmi_product_name,
            dmi_sys_vendor,
        }
    }

    // TODO: implement.
    fn new_windows() -> Self {
        unimplemented!()
    }

    /// Returns `true` if the given bios vendor matches that stored in the dmi data.
    pub fn is_bios_vendor(&self, vendor: &str) -> bool {
        if let Some(v) = self.dmi_bios_vendor.clone() {
            return v.contains(vendor);
        }

        false
    }

    /// Returns `true` if the given product name matches that stored in the dmi data.
    pub fn is_producy_name(&self, name: &str) -> bool {
        if let Some(v) = self.dmi_product_name.clone() {
            return v.contains(name);
        }

        false
    }

    /// Returns `true` if the given system vendor matches that stored in the dmi data.
    pub fn is_system_vendor(&self, vendor: &str) -> bool {
        if let Some(v) = self.dmi_sys_vendor.clone() {
            return v.contains(vendor);
        }

        false
    }
}

impl From<(&str, &str, &str)> for Smbios {
    fn from((dmi_bios_vendor, dmi_product_name, dmi_sys_vendor): (&str, &str, &str)) -> Self {
        Self {
            dmi_bios_vendor: dmi_bios_vendor
                .is_empty()
                .not()
                .then_some(dmi_bios_vendor.to_string()),
            dmi_product_name: dmi_product_name
                .is_empty()
                .not()
                .then_some(dmi_product_name.to_string()),
            dmi_sys_vendor: dmi_sys_vendor
                .is_empty()
                .not()
                .then_some(dmi_sys_vendor.to_string()),
        }
    }
}

impl Default for Smbios {
    fn default() -> Self {
        Self::new()
    }
}

// Attempts to read dmi data from sysfs.
//
// Returns `None` on error.
fn read_dmi_data(path: &str) -> Option<String> {
    let bytes = std::fs::read(path).unwrap_or_default();
    let data = String::from_utf8(bytes).unwrap_or_default();
    if data.is_empty() {
        None
    } else {
        Some(data.trim().to_lowercase())
    }
}
