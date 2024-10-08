/// Represents the differnent Cloud Run platforms.
pub enum CloudRunPlatform {
    Gen1,
    Gen2,
    Job,
}

/// Represents the supported GCP compute platforms.
pub enum ComputePlatform {
    CloudRun(CloudRunPlatform),
}
