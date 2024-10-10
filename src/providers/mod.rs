use std::cmp::Ordering;

pub mod aws;
pub mod azure;
pub mod gcp;
pub mod kubernetes;
pub mod nomad;
pub mod qemu;

/// Represents the currently supported compute platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePlatform {
    // AWS supported platforms.
    AwsEc2,
    AwsEcs,
    AwsFargate,
    AwsLambda,

    // Azure supported platforms.
    AzureContainerApp,
    AzureContainerAppJob,

    // GCP supported platforms.
    GcpCloudRunGen1,
    GcpCloudRunGen2,
    GcpCloudRunJob,

    // Generic supported platforms.
    Kubernetes,
    Nomad,
    Qemu,
}

impl PartialOrd for ComputePlatform {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }
}
