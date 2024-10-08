use anyhow::Result;

mod env;
mod providers;
use providers::*;

/// Represents the known platform details of a specific compute environment.
pub struct ComputeEnvironment {
    pub cloud_provider: Option<CloudProvider>,
    pub compute_platform: Option<ComputePlatform>,
}

// TODO: maybe this could be ComputeEnvironment::new()?
pub fn get_compute_environment() -> Result<ComputeEnvironment> {
    // TODO: plumb everything together.
    Ok(ComputeEnvironment {
        cloud_provider: None,
        compute_platform: None,
    })
}
