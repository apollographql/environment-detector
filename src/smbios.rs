const DMI_SYS_VENDOR: &str = "/sys/class/dmi/id/sys_vendor";

/// Represents data obtained from SMBIOS.
#[derive(Debug)]
pub struct Smbios {
    pub dmi_sys_vendor: Option<String>,
}

impl Smbios {
    // #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        let data = std::fs::read(DMI_SYS_VENDOR).unwrap_or_default();
        let vendor = String::from_utf8(data).unwrap_or_default();

        let dmi_sys_vendor = if vendor.is_empty() {
            None
        } else {
            Some(vendor.trim().to_lowercase())
        };

        Self { dmi_sys_vendor }
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn is_vendor(&self, vendor: &str) -> bool {
        if let Some(v) = self.dmi_sys_vendor.clone() {
            return v.contains(vendor);
        }

        false
    }
}
