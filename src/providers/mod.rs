use std::collections::HashSet;

use anyhow::{Context, Result};

mod aws;
mod azure;
mod env;
mod gcp;
mod kubernetes;
mod nomad;

const DMI_SYS_VENDOR: &str = "/sys/class/dmi/id/sys_vendor";

/// Represents the currently supported cloud providers.
#[derive(Clone, Copy, Debug)]
pub enum CloudProvider {
    Aws,
    Azure,
    Gcp,
}

/// Attempts to detect a cloud provider from both SMBIOS and environment wariable data.
pub fn get_cloud_provider() -> Result<Option<CloudProvider>> {
    // #[cfg(target_os = "linux")]
    let data = std::fs::read(DMI_SYS_VENDOR).context("error reading data from `dmi_sys_vendor`")?;
    let vendor = String::from_utf8(data).context("invalid value returned from `dmi_sys_vendor`")?;

    if vendor.contains("Amazon") {
        return Ok(Some(CloudProvider::Aws));
    } else if vendor.contains("Google") {
        return Ok(Some(CloudProvider::Gcp));
    } else if vendor.contains("Microsoft") {
        return Ok(Some(CloudProvider::Azure));
    }

    Ok(None)
}

/// Represents the currently supported compute platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePlatform {
    Aws(aws::ComputePlatform),
    Azure(azure::ComputePlatform),
    Gcp(gcp::ComputePlatform),
    Kubernetes,
    Nomad,
    Qemu,
}

/// Attempts to detect a compute platform from both SMBIOS and environment wariable data.
pub fn get_compute_platform(cloud_provider: Option<CloudProvider>) -> Option<ComputePlatform> {
    // Compile a list of supported env vars that are present.
    let env_vars: HashSet<&str> = env::PLATFORM_ENV_VARS
        .iter()
        .filter(|var| crate::env::hasenv(var))
        .fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });

    // If we have a cloud provider, use specific identifiers for detecing the compute platform.
    if let Some(provider) = cloud_provider {
        match provider {
            CloudProvider::Aws => {
                return aws::detect_compute_platform(&env_vars).map(ComputePlatform::Aws)
            }
            CloudProvider::Azure => {
                return azure::detect_compute_platform(&env_vars).map(ComputePlatform::Azure)
            }
            CloudProvider::Gcp => {
                return gcp::detect_compute_platform(&env_vars).map(ComputePlatform::Gcp)
            }
        }
    }

    // If we weren't able to match a cloud specific platform, check for general platforms too.
    if let Some(k) = kubernetes::detect_compute_platform(&env_vars) {
        return Some(k);
    }
    if let Some(n) = nomad::detect_compute_platform(&env_vars) {
        return Some(n);
    }

    // We weren't able to match a platform.
    None
}
