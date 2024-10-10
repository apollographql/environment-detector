const DMI_SYS_VENDOR: &str = "/sys/class/dmi/id/sys_vendor";

/// Represents data obtained from SMBIOS.
#[derive(Debug)]
pub struct Smbios {
    pub dmi_sys_vendor: Option<String>,
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
        let data = std::fs::read(DMI_SYS_VENDOR).unwrap_or_default();
        let vendor = String::from_utf8(data).unwrap_or_default();

        let dmi_sys_vendor = if vendor.is_empty() {
            None
        } else {
            Some(vendor.trim().to_lowercase())
        };

        Self { dmi_sys_vendor }
    }

    fn new_windows() -> Self {
        unimplemented!()
    }

    pub fn is_vendor(&self, vendor: &str) -> bool {
        if let Some(v) = self.dmi_sys_vendor.clone() {
            return v.contains(vendor);
        }

        false
    }
}

impl Default for Smbios {
    fn default() -> Self {
        Self::new()
    }
}
