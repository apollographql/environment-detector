use std::ops::Not;

/// Represents data obtained from SMBIOS.
#[derive(Debug, Default)]
pub struct Smbios {
    dmi_bios_vendor: Option<String>,
    dmi_product_name: Option<String>,
    dmi_sys_vendor: Option<String>,
}

impl Smbios {
    #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        Self {
            dmi_bios_vendor: read_dmi_data("/sys/class/dmi/id/bios_vendor"),
            dmi_product_name: read_dmi_data("/sys/class/dmi/id/product_name"),
            dmi_sys_vendor: read_dmi_data("/sys/class/dmi/id/sys_vendor"),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn new() -> Self {
        use serde::Deserialize;
        use wmi::{COMLibrary, WMIConnection};

        #[derive(Deserialize)]
        #[serde(rename = "Win32_ComputerSystemProduct")]
        #[serde(rename_all = "PascalCase")]
        struct ComputerSystemProduct {
            name: String,
            vendor: String,
        }

        let Ok(com) = COMLibrary::new() else {
            return Self::default();
        };

        let Ok(wmi_con) = WMIConnection::new(com) else {
            return Self::default();
        };

        let Ok::<Vec<ComputerSystemProduct>, _>(results) = wmi_con.query() else {
            return Self::default();
        };

        let Some(product) = results.get(0) else {
            return Self::default();
        };

        Self {
            dmi_bios_vendor: Some(product.vendor.to_string()),
            dmi_product_name: Some(product.name.to_string()),
            dmi_sys_vendor: None,
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns `true` if the given bios vendor matches that stored in the dmi data.
    pub fn is_bios_vendor(&self, vendor: &str) -> bool {
        if vendor.is_empty() {
            return false;
        }

        if let Some(v) = self.dmi_bios_vendor.as_ref() {
            return v.contains(vendor);
        }

        false
    }

    /// Returns `true` if the given product name matches that stored in the dmi data.
    pub fn is_product_name(&self, product_name: &str) -> bool {
        if product_name.is_empty() {
            return false;
        }

        if let Some(v) = self.dmi_product_name.as_ref() {
            return v.contains(product_name);
        }

        false
    }

    /// Returns `true` if the given system vendor matches that stored in the dmi data.
    pub fn is_system_vendor(&self, vendor: &str) -> bool {
        if vendor.is_empty() {
            return false;
        }

        if let Some(v) = self.dmi_sys_vendor.as_ref() {
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

// Attempts to read dmi data from sysfs.
//
// Returns `None` on error.
#[cfg(target_os = "linux")]
fn read_dmi_data(path: &str) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    let data = String::from_utf8(bytes).ok()?;
    data.is_empty().then(|| data.trim().to_lowercase())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::Smbios;

    #[rstest]
    #[case("bios_vendor", true)]
    #[case("", false)]
    fn test_is_bios_vendor(#[case] vendor: &str, #[case] expected: bool) {
        let smbios = Smbios::from(("bios_vendor", "", ""));
        let actual = smbios.is_bios_vendor(vendor);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("product_name", true)]
    #[case("", false)]
    fn test_is_product_name(#[case] product_name: &str, #[case] expected: bool) {
        let smbios = Smbios::from(("", "product_name", ""));
        let actual = smbios.is_product_name(product_name);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("system_vendor", true)]
    #[case("", false)]
    fn test_is_system_vendor(#[case] vendor: &str, #[case] expected: bool) {
        let smbios = Smbios::from(("", "", "system_vendor"));
        let actual = smbios.is_system_vendor(vendor);
        assert_eq!(expected, actual);
    }
}
