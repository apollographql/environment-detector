use anyhow::Result;

/// Represents the differnent Cloud Run platforms.
#[derive(Clone, Debug)]
pub enum CloudRunPlatform {
    Gen1,
    Gen2,
    Job,
}

/// Represents the supported GCP compute platforms.
#[derive(Clone, Debug)]
pub enum ComputePlatform {
    CloudRun(CloudRunPlatform),
}

pub(crate) fn detect_compute_platform() -> Result<Option<ComputePlatform>> {
    Ok(None)
}
