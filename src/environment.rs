use std::fmt::Display;

use crate::{detector::Detector, env_vars, smbios};

/// Represents the currently supported compute platforms.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ComputeEnvironment {
    // AWS supported platforms.
    AwsEc2,
    AwsEcs,
    // AwsFargate,
    AwsLambda,
    AwsKubernetes,
    AwsNomad,

    // Azure supported platforms.
    AzureContainerApps,
    AzureContainerAppsJob,
    AzureContainerInstance,
    AzureKubernetes,
    AzureVM,
    AzureNomad,

    // GCP supported platforms.
    GcpCloudRunGen1,
    GcpCloudRunGen2,
    GcpCloudRunJob,
    GcpComputeEngine,
    GcpKubernetes,
    GcpNomad,

    // Generic supported platforms.
    Kubernetes,
    Nomad,
    Qemu,
}

impl ComputeEnvironment {
    pub(crate) fn iter() -> ComputeEnvironmentIter {
        ComputeEnvironmentIter { idx: 0 }
    }

    pub(crate) fn detector(&self) -> Detector {
        match self {
            ComputeEnvironment::AwsEc2 => Detector::new(*self, smbios::AWS, env_vars::EMPTY),
            ComputeEnvironment::AwsEcs => Detector::new(*self, smbios::EMPTY, env_vars::AWS_ECS),
            ComputeEnvironment::AwsLambda => {
                Detector::new(*self, smbios::EMPTY, env_vars::AWS_LAMBDA)
            }
            ComputeEnvironment::AwsKubernetes => {
                Detector::new(*self, smbios::AWS, env_vars::KUBERNETES)
            }
            ComputeEnvironment::AwsNomad => Detector::new(*self, smbios::AWS, env_vars::NOMAD),
            ComputeEnvironment::AzureContainerApps => {
                Detector::new(*self, smbios::AZURE, env_vars::AZURE_CONTAINER_APPS)
            }
            ComputeEnvironment::AzureContainerAppsJob => {
                Detector::new(*self, smbios::AZURE, env_vars::AZURE_CONTAINER_APPS_JOB)
            }
            Self::AzureContainerInstance => {
                Detector::new(*self, smbios::EMPTY, env_vars::AZURE_CONTAINER_INSTANCE)
            }
            ComputeEnvironment::AzureKubernetes => {
                Detector::new(*self, smbios::AZURE, env_vars::KUBERNETES)
            }
            ComputeEnvironment::AzureVM => Detector::new(*self, smbios::AZURE, env_vars::EMPTY),
            ComputeEnvironment::AzureNomad => Detector::new(*self, smbios::AZURE, env_vars::NOMAD),
            ComputeEnvironment::GcpCloudRunGen1 => {
                Detector::new(*self, smbios::EMPTY, env_vars::GCP_CLOUD_RUN_SERVICE)
            }
            ComputeEnvironment::GcpCloudRunGen2 => {
                Detector::new(*self, smbios::GCP, env_vars::GCP_CLOUD_RUN_SERVICE)
            }
            ComputeEnvironment::GcpCloudRunJob => {
                Detector::new(*self, smbios::GCP, env_vars::GCP_CLOUD_RUN_JOB)
            }
            ComputeEnvironment::GcpComputeEngine => {
                Detector::new(*self, smbios::GCP, env_vars::EMPTY)
            }
            ComputeEnvironment::GcpKubernetes => {
                Detector::new(*self, smbios::GCP, env_vars::KUBERNETES)
            }
            ComputeEnvironment::GcpNomad => Detector::new(*self, smbios::GCP, env_vars::NOMAD),
            ComputeEnvironment::Kubernetes => {
                Detector::new(*self, smbios::EMPTY, env_vars::KUBERNETES)
            }
            ComputeEnvironment::Nomad => Detector::new(*self, smbios::EMPTY, env_vars::NOMAD),
            ComputeEnvironment::Qemu => Detector::new(*self, smbios::QEMU, env_vars::EMPTY),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ComputeEnvironment::AwsEc2 => "AWS EC2",
            ComputeEnvironment::AwsEcs => "AWS ECS",
            ComputeEnvironment::AwsLambda => "AWS Lambda",
            ComputeEnvironment::AwsKubernetes => "Kubernetes on AWS",
            ComputeEnvironment::AwsNomad => "Nomad on AWS",
            ComputeEnvironment::AzureContainerApps => "Azure Container Apps",
            ComputeEnvironment::AzureContainerAppsJob => "Azure Container Apps Job",
            ComputeEnvironment::AzureContainerInstance => "Azure Container Instance",
            ComputeEnvironment::AzureKubernetes => "Kubernetes on Azure",
            ComputeEnvironment::AzureVM => "Azure VM",
            ComputeEnvironment::AzureNomad => "Nomad on Azure",
            ComputeEnvironment::GcpCloudRunGen1 => "Google Cloud Run (Gen1)",
            ComputeEnvironment::GcpCloudRunGen2 => "Google Cloud Run (Gen2)",
            ComputeEnvironment::GcpCloudRunJob => "Google Cloud Run (Job)",
            ComputeEnvironment::GcpComputeEngine => "Google Compute Engine",
            ComputeEnvironment::GcpKubernetes => "Kubernetes on Google Cloud",
            ComputeEnvironment::GcpNomad => "Nomad on Google Cloud",
            ComputeEnvironment::Kubernetes => "Kubernetes",
            ComputeEnvironment::Nomad => "Nomad",
            ComputeEnvironment::Qemu => "QEMU",
        }
    }

    pub fn cloud_provider(&self) -> Option<CloudProvider> {
        match self {
            ComputeEnvironment::AwsEc2
            | ComputeEnvironment::AwsEcs
            | ComputeEnvironment::AwsLambda
            | ComputeEnvironment::AwsKubernetes
            | ComputeEnvironment::AwsNomad => Some(CloudProvider::Aws),
            ComputeEnvironment::AzureContainerApps
            | ComputeEnvironment::AzureContainerAppsJob
            | ComputeEnvironment::AzureContainerInstance
            | ComputeEnvironment::AzureKubernetes
            | ComputeEnvironment::AzureVM
            | ComputeEnvironment::AzureNomad => Some(CloudProvider::Azure),
            ComputeEnvironment::GcpCloudRunGen1
            | ComputeEnvironment::GcpCloudRunGen2
            | ComputeEnvironment::GcpCloudRunJob
            | ComputeEnvironment::GcpComputeEngine
            | ComputeEnvironment::GcpKubernetes
            | ComputeEnvironment::GcpNomad => Some(CloudProvider::GoogleCloud),
            ComputeEnvironment::Kubernetes
            | ComputeEnvironment::Nomad
            | ComputeEnvironment::Qemu => None,
        }
    }
}

impl Display for ComputeEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

pub(crate) struct ComputeEnvironmentIter {
    idx: usize,
}

impl ComputeEnvironmentIter {
    fn get(&self, idx: usize) -> Option<ComputeEnvironment> {
        match idx {
            0usize => Some(ComputeEnvironment::AwsEc2),
            1usize => Some(ComputeEnvironment::AwsEcs),
            2usize => Some(ComputeEnvironment::AwsLambda),
            3usize => Some(ComputeEnvironment::AwsKubernetes),
            4usize => Some(ComputeEnvironment::AwsNomad),
            5usize => Some(ComputeEnvironment::AzureContainerApps),
            6usize => Some(ComputeEnvironment::AzureContainerAppsJob),
            7usize => Some(ComputeEnvironment::AzureContainerInstance),
            8usize => Some(ComputeEnvironment::AzureKubernetes),
            9usize => Some(ComputeEnvironment::AzureVM),
            10usize => Some(ComputeEnvironment::AzureNomad),
            11usize => Some(ComputeEnvironment::GcpCloudRunGen1),
            12usize => Some(ComputeEnvironment::GcpCloudRunGen2),
            13usize => Some(ComputeEnvironment::GcpCloudRunJob),
            14usize => Some(ComputeEnvironment::GcpComputeEngine),
            15usize => Some(ComputeEnvironment::GcpKubernetes),
            16usize => Some(ComputeEnvironment::GcpNomad),
            17usize => Some(ComputeEnvironment::Kubernetes),
            18usize => Some(ComputeEnvironment::Nomad),
            19usize => Some(ComputeEnvironment::Qemu),
            _ => None,
        }
    }
}

impl Iterator for ComputeEnvironmentIter {
    type Item = ComputeEnvironment;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.get(self.idx);
        self.idx += 1;
        ret
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CloudProvider {
    Aws,
    Azure,
    GoogleCloud,
}

impl CloudProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            CloudProvider::Aws => "AWS",
            CloudProvider::Azure => "Azure",
            CloudProvider::GoogleCloud => "Google Cloud",
        }
    }
}

impl Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specificity::Specificity as _;
    use rstest::{fixture, rstest};
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[fixture]
    fn expected_matrix() -> HashMap<(ComputeEnvironment, ComputeEnvironment), Option<Ordering>> {
        let envs: HashMap<_, _> = ComputeEnvironment::iter().enumerate().collect();
        let matrix = include_str!("tests/specificity_matrix.txt");
        matrix
            .split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                let envs = &envs;
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (*envs.get(&y).unwrap(), *envs.get(&x).unwrap()),
                        match c {
                            '-' => Some(Ordering::Less),
                            '=' => Some(Ordering::Equal),
                            '+' => Some(Ordering::Greater),
                            _ => None,
                        },
                    )
                })
            })
            .collect()
    }

    #[rstest]
    fn test_specificity(
        #[values(
            ComputeEnvironment::AwsEc2,
            ComputeEnvironment::AwsEcs,
            ComputeEnvironment::AwsLambda,
            ComputeEnvironment::AwsKubernetes,
            ComputeEnvironment::AwsNomad,
            ComputeEnvironment::AzureContainerApps,
            ComputeEnvironment::AzureContainerAppsJob,
            ComputeEnvironment::AzureContainerInstance,
            ComputeEnvironment::AzureKubernetes,
            ComputeEnvironment::AzureVM,
            ComputeEnvironment::AzureNomad,
            ComputeEnvironment::GcpCloudRunGen1,
            ComputeEnvironment::GcpCloudRunGen2,
            ComputeEnvironment::GcpCloudRunJob,
            ComputeEnvironment::GcpComputeEngine,
            ComputeEnvironment::GcpKubernetes,
            ComputeEnvironment::GcpNomad,
            ComputeEnvironment::Kubernetes,
            ComputeEnvironment::Nomad,
            ComputeEnvironment::Qemu
        )]
        left: ComputeEnvironment,
        #[values(
            ComputeEnvironment::AwsEc2,
            ComputeEnvironment::AwsEcs,
            ComputeEnvironment::AwsLambda,
            ComputeEnvironment::AwsKubernetes,
            ComputeEnvironment::AwsNomad,
            ComputeEnvironment::AzureContainerApps,
            ComputeEnvironment::AzureContainerAppsJob,
            ComputeEnvironment::AzureContainerInstance,
            ComputeEnvironment::AzureKubernetes,
            ComputeEnvironment::AzureVM,
            ComputeEnvironment::AzureNomad,
            ComputeEnvironment::GcpCloudRunGen1,
            ComputeEnvironment::GcpCloudRunGen2,
            ComputeEnvironment::GcpCloudRunJob,
            ComputeEnvironment::GcpComputeEngine,
            ComputeEnvironment::GcpKubernetes,
            ComputeEnvironment::GcpNomad,
            ComputeEnvironment::Kubernetes,
            ComputeEnvironment::Nomad,
            ComputeEnvironment::Qemu
        )]
        right: ComputeEnvironment,
        expected_matrix: HashMap<(ComputeEnvironment, ComputeEnvironment), Option<Ordering>>,
    ) {
        let expected = expected_matrix.get(&(left, right)).cloned().flatten();

        let result = left.detector().specificity_cmp(&right.detector());

        assert_eq!(expected, result);
    }
}
