use anyhow::Result;

pub mod env;
mod providers;
use providers::*;

/// Represents the known platform details of a specific compute environment.
pub struct ComputeEnvironment {
    pub cloud_provider: Option<CloudProvider>,
    pub compute_platform: Option<ComputePlatform>,
}

pub fn get_compute_environment() -> Result<ComputeEnvironment> {
    let cloud_provider = get_cloud_provider()?;
    let compute_platform = get_compute_platform(cloud_provider);

    Ok(ComputeEnvironment {
        cloud_provider,
        compute_platform,
    })
}
