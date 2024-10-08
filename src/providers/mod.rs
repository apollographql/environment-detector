mod aws;
mod azure;
mod gcp;

/// Represents the currently supported cloud providers.
pub enum CloudProvider {
    Aws,
    Azure,
    Gcp,
}

/// Represents the currently supported compute platforms.
pub enum ComputePlatform {
    Aws(aws::ComputePlatform),
    Azure(azure::ComputePlatform),
    Gcp(gcp::ComputePlatform),
    Kubernetes,
    Nomad,
    VirtualMachine,
}
