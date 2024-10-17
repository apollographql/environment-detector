use std::cmp::Ordering;

use crate::specificity::{OrderingExt, Specificity};

pub const AWS: SmbiosPattern = SmbiosPattern::new()
    .with_bios_vendor("amazon")
    .with_sys_vendor("amazon");
pub const AZURE: SmbiosPattern = SmbiosPattern::new()
    .with_bios_vendor("microsoft")
    .with_sys_vendor("microsoft");
pub const EMPTY: SmbiosPattern = SmbiosPattern::new();
pub const GCP: SmbiosPattern = SmbiosPattern::new()
    .with_bios_vendor("google")
    .with_sys_vendor("google");
pub const QEMU: SmbiosPattern = SmbiosPattern::new().with_sys_vendor("qemu");

/// Represents data obtained from SMBIOS.
#[derive(Debug, Default, Clone)]
pub struct Smbios {
    bios_vendor: Option<String>,
    product_name: Option<String>,
    sys_vendor: Option<String>,
}

impl Smbios {
    #[cfg(target_os = "linux")]
    pub fn detect() -> Self {
        Self {
            bios_vendor: read_dmi_data("/sys/class/dmi/id/bios_vendor"),
            product_name: read_dmi_data("/sys/class/dmi/id/product_name"),
            sys_vendor: read_dmi_data("/sys/class/dmi/id/sys_vendor"),
        }
    }

    #[cfg(target_os = "windows")]
    pub fn detect() -> Self {
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
            bios_vendor: Some(product.vendor.trim().to_lowercase()),
            product_name: Some(product.name.trim().to_lowercase()),
            sys_vendor: None,
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    pub fn detect() -> Self {
        Self::default()
    }
}

impl From<SmbiosPattern> for Smbios {
    fn from(value: SmbiosPattern) -> Self {
        Self {
            bios_vendor: value.bios_vendor.map(ToString::to_string),
            product_name: value.product_name.map(ToString::to_string),
            sys_vendor: value.sys_vendor.map(ToString::to_string),
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
    if data.is_empty() {
        None
    } else {
        Some(data.trim().to_string())
    }
}

#[derive(Default, Debug, Clone)]
pub struct SmbiosPattern {
    bios_vendor: Option<&'static str>,
    product_name: Option<&'static str>,
    sys_vendor: Option<&'static str>,
}

impl SmbiosPattern {
    /// Returns how many SMBIOS data points are matching the pattern
    ///
    /// This returns a score from 0 to 16384
    pub fn detect(&self, smbios: &Smbios) -> u16 {
        let mut total = 0;
        let mut found = 0;
        if let Some(bios_vendor) = self.bios_vendor {
            total += 1;
            if smbios
                .bios_vendor
                .as_ref()
                .map(|detected_vendor| detected_vendor.to_lowercase().contains(bios_vendor))
                .unwrap_or(false)
            {
                found += 1;
            }
        }

        if let Some(product_name) = self.product_name {
            total += 1;
            if smbios
                .product_name
                .as_ref()
                .map(|detected_vendor| detected_vendor.to_lowercase().contains(product_name))
                .unwrap_or(false)
            {
                found += 1;
            }
        }

        if let Some(sys_vendor) = self.sys_vendor {
            total += 1;
            if smbios
                .sys_vendor
                .as_ref()
                .map(|detected_vendor| detected_vendor.to_lowercase().contains(sys_vendor))
                .unwrap_or(false)
            {
                found += 1;
            }
        }

        if total == 0 {
            8192
        } else {
            found * 16384 / total
        }
    }

    pub const fn new() -> Self {
        Self {
            bios_vendor: None,
            product_name: None,
            sys_vendor: None,
        }
    }

    pub const fn with_bios_vendor(self, bios_vendor: &'static str) -> Self {
        Self {
            bios_vendor: Some(bios_vendor),
            ..self
        }
    }

    #[allow(unused)]
    pub const fn with_product_name(self, product_name: &'static str) -> Self {
        Self {
            product_name: Some(product_name),
            ..self
        }
    }

    pub const fn with_sys_vendor(self, sys_vendor: &'static str) -> Self {
        Self {
            sys_vendor: Some(sys_vendor),
            ..self
        }
    }
}

impl Specificity for SmbiosPattern {
    fn specificity_cmp(&self, other: &Self) -> Option<Ordering> {
        let bios_vendor = self.bios_vendor.specificity_cmp(&other.bios_vendor);
        let product_name = self.product_name.specificity_cmp(&other.product_name);
        let sys_vendor = self.sys_vendor.specificity_cmp(&other.sys_vendor);

        bios_vendor
            .merge_specificity(product_name)
            .merge_specificity(sys_vendor)
    }
}
