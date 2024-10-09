use anyhow::Result;

mod aws;
mod azure;
mod gcp;

/// Represents the currently supported cloud providers.
#[derive(Clone, Copy, Debug)]
pub enum CloudProvider {
    Aws,
    Azure,
    Gcp,
}

/// Attempts to detect a cloud provider from both SMBIOS and environment wariable data.
pub fn get_cloud_provider() -> Result<Option<CloudProvider>> {
    // TODO: read data from dmidecode and parse `dmi_sys_vendor`:
    // - Amazon
    // - Google
    // - Microsoft Corporation
    Ok(None)
}

/// Represents the currently supported compute platforms.
#[derive(Clone, Debug)]
pub enum ComputePlatform {
    Aws(aws::ComputePlatform),
    Azure(azure::ComputePlatform),
    Gcp(gcp::ComputePlatform),
    Kubernetes,
    Nomad,
    Qemu,
}

/// Attempts to detect a compute platform from both SMBIOS and environment wariable data.
pub fn get_compute_platform(
    cloud_provider: Option<CloudProvider>,
) -> Result<Option<ComputePlatform>> {
    // If we have a cloud provider, use specific identifiers for detecing the compute platform.
    if let Some(provider) = cloud_provider {
        match provider {
            CloudProvider::Aws => {
                let platform = aws::detect_compute_platform()?;
                if let Some(p) = platform {
                    return Ok(Some(ComputePlatform::Aws(p)));
                }
            }
            CloudProvider::Azure => {
                let platform = azure::detect_compute_platform()?;
                if let Some(p) = platform {
                    return Ok(Some(ComputePlatform::Azure(p)));
                }
            }
            CloudProvider::Gcp => {
                let platform = gcp::detect_compute_platform()?;
                if let Some(p) = platform {
                    return Ok(Some(ComputePlatform::Gcp(p)));
                }
            }
        }
    // If we don't have a cloud provider, look for a standalone platform.
    } else {
        todo!("implement non-cloud provider platform detection");
    }

    Ok(None)
}
