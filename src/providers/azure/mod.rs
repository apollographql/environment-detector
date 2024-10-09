use anyhow::Result;

/// Represents the supported Azure compute platforms.
#[derive(Clone, Debug)]
pub enum ComputePlatform {
    ContainerApp,
    ContainerAppJob,
}

pub(crate) fn detect_compute_platform() -> Result<Option<ComputePlatform>> {
    Ok(None)
}
