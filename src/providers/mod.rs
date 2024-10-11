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
    AzureContainerApps,
    AzureContainerAppsJob,
    AzureVM,

    // GCP supported platforms.
    GcpCloudRunGen1,
    GcpCloudRunGen2,
    GcpCloudRunJob,
    GcpComputeEngine,

    // Generic supported platforms.
    Kubernetes,
    Nomad,
    Qemu,
}

impl ComputePlatform {
    /// Returns `true` if this detector is a superset of another detector.
    fn is_superset_of(&self, other: &Self) -> bool {
        // Kubernetes/Nomad are subsets of:
        // - AWS EC2
        // - AWS Fargate
        // - Azure Container Apps
        // - Azure Container Apps Job
        // - Azure VM
        // - GCP Compute Engine
        if (self == &ComputePlatform::Kubernetes || self == &ComputePlatform::Nomad)
            && (other == &ComputePlatform::AwsEc2
                || other == &ComputePlatform::AwsFargate
                || other == &ComputePlatform::AzureContainerApps
                || other == &ComputePlatform::AzureContainerAppsJob
                || other == &ComputePlatform::AzureVM
                || other == &ComputePlatform::GcpComputeEngine)
        {
            return true;
        }

        // AWS ECS is a superset of AWS EC2/Fargate.
        if self == &ComputePlatform::AwsEcs
            && (other == &ComputePlatform::AwsEc2 || other == &ComputePlatform::AwsFargate)
        {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ComputePlatform;

    #[rstest]
    #[case(ComputePlatform::Kubernetes, ComputePlatform::AwsEc2, true)]
    #[case(ComputePlatform::Kubernetes, ComputePlatform::AwsFargate, true)]
    #[case(ComputePlatform::Kubernetes, ComputePlatform::AzureContainerApps, true)]
    #[case(
        ComputePlatform::Kubernetes,
        ComputePlatform::AzureContainerAppsJob,
        true
    )]
    #[case(ComputePlatform::Kubernetes, ComputePlatform::AzureVM, true)]
    #[case(ComputePlatform::Kubernetes, ComputePlatform::GcpComputeEngine, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::AwsEc2, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::AwsFargate, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::AzureContainerApps, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::AzureContainerAppsJob, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::AzureVM, true)]
    #[case(ComputePlatform::Nomad, ComputePlatform::GcpComputeEngine, true)]
    #[case(ComputePlatform::AwsEcs, ComputePlatform::AwsEc2, true)]
    #[case(ComputePlatform::AwsEcs, ComputePlatform::AwsFargate, true)]
    fn test_is_superset_of(
        #[case] platform_a: ComputePlatform,
        #[case] platform_b: ComputePlatform,
        #[case] expected_superset: bool,
    ) {
        let is_superset = platform_a.is_superset_of(&platform_b);
        assert_eq!(expected_superset, is_superset);
    }
}
