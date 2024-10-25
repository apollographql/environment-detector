use std::fmt::Display;

use crate::{detector::Detector, env_vars, smbios};

/// Supported compute environments that can be detected by this crate
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ComputeEnvironment {
    // AWS supported platforms.
    /// Amazon Elastic Compute Cloud (EC2)
    AwsEc2,
    /// Amazon Elastic Container Service (ECS)
    AwsEcs,
    /// AWS Lambda
    AwsLambda,
    /// Kubernetes on AWS
    AwsKubernetes,
    /// Nomad on AWS
    AwsNomad,

    // Azure supported platforms.
    /// Azure Containers Apps
    AzureContainerApps,
    /// Azure Container Apps Job
    AzureContainerAppsJob,
    /// Azure Container Instance
    AzureContainerInstance,
    /// Kubernetes on Azure
    AzureKubernetes,
    /// Azure VM
    AzureVM,
    /// Nomad on Azure
    AzureNomad,

    // GCP supported platforms.
    /// Google Cloud Run (Gen1)
    GcpCloudRunGen1,
    /// Google Cloud Run (Gen2)
    GcpCloudRunGen2,
    /// Google Cloud Run (Job)
    GcpCloudRunJob,
    /// Google Compute Engine
    GcpComputeEngine,
    /// Kubernetes on Google Cloud
    GcpKubernetes,
    /// Nomad on Google Cloud
    GcpNomad,

    // Generic supported platforms.
    /// Kubernetes
    Kubernetes,
    /// Nomad
    Nomad,
    /// QEMU
    Qemu,

    #[cfg(test)]
    Testing,
}

impl ComputeEnvironment {
    pub(crate) fn iter() -> ComputeEnvironmentIter {
        ComputeEnvironmentIter { idx: 0 }
    }

    pub(crate) fn detector(&self) -> Detector {
        match self {
            Self::AwsEc2 => Detector::new(*self, smbios::AWS, env_vars::EMPTY),
            Self::AwsEcs => Detector::new(*self, smbios::EMPTY, env_vars::AWS_ECS),
            Self::AwsLambda => Detector::new(*self, smbios::EMPTY, env_vars::AWS_LAMBDA),
            Self::AwsKubernetes => Detector::new(*self, smbios::AWS, env_vars::KUBERNETES),
            Self::AwsNomad => Detector::new(*self, smbios::AWS, env_vars::NOMAD),
            Self::AzureContainerApps => {
                Detector::new(*self, smbios::AZURE, env_vars::AZURE_CONTAINER_APPS)
            }
            Self::AzureContainerAppsJob => {
                Detector::new(*self, smbios::AZURE, env_vars::AZURE_CONTAINER_APPS_JOB)
            }
            Self::AzureContainerInstance => {
                Detector::new(*self, smbios::EMPTY, env_vars::AZURE_CONTAINER_INSTANCE)
            }
            Self::AzureKubernetes => Detector::new(*self, smbios::AZURE, env_vars::KUBERNETES),
            Self::AzureVM => Detector::new(*self, smbios::AZURE, env_vars::EMPTY),
            Self::AzureNomad => Detector::new(*self, smbios::AZURE, env_vars::NOMAD),
            Self::GcpCloudRunGen1 => {
                Detector::new(*self, smbios::EMPTY, env_vars::GCP_CLOUD_RUN_SERVICE)
            }
            Self::GcpCloudRunGen2 => {
                Detector::new(*self, smbios::GCP, env_vars::GCP_CLOUD_RUN_SERVICE)
            }
            Self::GcpCloudRunJob => Detector::new(*self, smbios::GCP, env_vars::GCP_CLOUD_RUN_JOB),
            Self::GcpComputeEngine => Detector::new(*self, smbios::GCP, env_vars::EMPTY),
            Self::GcpKubernetes => Detector::new(*self, smbios::GCP, env_vars::KUBERNETES),
            Self::GcpNomad => Detector::new(*self, smbios::GCP, env_vars::NOMAD),
            Self::Kubernetes => Detector::new(*self, smbios::EMPTY, env_vars::KUBERNETES),
            Self::Nomad => Detector::new(*self, smbios::EMPTY, env_vars::NOMAD),
            Self::Qemu => Detector::new(*self, smbios::QEMU, env_vars::EMPTY),

            #[cfg(test)]
            Self::Testing => Detector::new(*self, smbios::EMPTY, env_vars::EMPTY),
        }
    }

    /// Static str representation of the [`ComputeEnvironment`]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AwsEc2 => "AWS EC2",
            Self::AwsEcs => "AWS ECS",
            Self::AwsLambda => "AWS Lambda",
            Self::AwsKubernetes => "Kubernetes on AWS",
            Self::AwsNomad => "Nomad on AWS",
            Self::AzureContainerApps => "Azure Container Apps",
            Self::AzureContainerAppsJob => "Azure Container Apps Job",
            Self::AzureContainerInstance => "Azure Container Instance",
            Self::AzureKubernetes => "Kubernetes on Azure",
            Self::AzureVM => "Azure VM",
            Self::AzureNomad => "Nomad on Azure",
            Self::GcpCloudRunGen1 => "Google Cloud Run (Gen1)",
            Self::GcpCloudRunGen2 => "Google Cloud Run (Gen2)",
            Self::GcpCloudRunJob => "Google Cloud Run (Job)",
            Self::GcpComputeEngine => "Google Compute Engine",
            Self::GcpKubernetes => "Kubernetes on Google Cloud",
            Self::GcpNomad => "Nomad on Google Cloud",
            Self::Kubernetes => "Kubernetes",
            Self::Nomad => "Nomad",
            Self::Qemu => "QEMU",

            #[cfg(test)]
            Self::Testing => "Testing",
        }
    }

    /// Compute Platform code
    ///
    /// This corresponds to the `cloud.platform` attribute in OpenTelemetry semantic conventions
    /// where possible.
    ///
    /// For Kubernetes on Cloud Providers, this always assumes that someone is using the
    /// corresponding managed service (e.g. AWS EKS, AKS, or GKE).
    ///
    /// This may also return one of the following values for some environments:
    ///
    /// * `kubernetes`
    /// * `nomad`
    /// * `qemu`
    ///
    /// See <https://opentelemetry.io/docs/specs/semconv/attributes-registry/cloud/>
    pub fn platform_code(&self) -> &'static str {
        match self {
            Self::AwsEc2 => "aws_ec2",
            Self::AwsEcs => "aws_ecs",
            Self::AwsLambda => "aws_lambda",
            // We're assuming Kubernetes on AWS = EKS
            Self::AwsKubernetes => "aws_eks",
            Self::AwsNomad => "nomad",
            Self::AzureContainerApps => "azure_container_apps",
            Self::AzureContainerAppsJob => "azure_container_apps",
            Self::AzureContainerInstance => "azure_container_instances",
            // We're assuming Kubernetes on Azure = AKS
            Self::AzureKubernetes => "azure_aks",
            Self::AzureVM => "azure_vm",
            Self::AzureNomad => "nomad",
            Self::GcpCloudRunGen1 => "gcp_cloud_run",
            Self::GcpCloudRunGen2 => "gcp_cloud_run",
            Self::GcpCloudRunJob => "gcp_cloud_run",
            Self::GcpComputeEngine => "gcp_compute_engine",
            // We're assuming Kubernetes on GCP = GKE
            Self::GcpKubernetes => "gcp_kubernetes_engine",
            Self::GcpNomad => "nomad",
            Self::Kubernetes => "kubernetes",
            Self::Nomad => "nomad",
            Self::Qemu => "qemu",

            #[cfg(test)]
            Self::Testing => "testing",
        }
    }

    /// [`CloudProvider`] for this compute environment
    pub fn cloud_provider(&self) -> Option<CloudProvider> {
        match self {
            Self::AwsEc2
            | Self::AwsEcs
            | Self::AwsLambda
            | Self::AwsKubernetes
            | Self::AwsNomad => Some(CloudProvider::Aws),
            Self::AzureContainerApps
            | Self::AzureContainerAppsJob
            | Self::AzureContainerInstance
            | Self::AzureKubernetes
            | Self::AzureVM
            | Self::AzureNomad => Some(CloudProvider::Azure),
            Self::GcpCloudRunGen1
            | Self::GcpCloudRunGen2
            | Self::GcpCloudRunJob
            | Self::GcpComputeEngine
            | Self::GcpKubernetes
            | Self::GcpNomad => Some(CloudProvider::GoogleCloud),
            Self::Kubernetes | Self::Nomad | Self::Qemu => None,

            #[cfg(test)]
            Self::Testing => None,
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

/// Supported cloud providers that can be detected by this crate.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CloudProvider {
    /// Amazon Web Services
    Aws,
    /// Microsoft Azure
    Azure,
    /// Google Cloud Platform
    GoogleCloud,
}

impl CloudProvider {
    /// Static str representation of the [`CloudProvider`].
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Aws => "AWS",
            Self::Azure => "Azure",
            Self::GoogleCloud => "Google Cloud",
        }
    }
    /// Cloud Provider code.
    ///
    /// This corresponds to the `cloud.provider` attribute in OpenTelemetry semantic conventions.
    ///
    /// See: <https://opentelemetry.io/docs/specs/semconv/attributes-registry/cloud/>
    pub fn code(&self) -> &'static str {
        match self {
            Self::Aws => "aws",
            Self::Azure => "azure",
            Self::GoogleCloud => "gcp",
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
    use std::cmp::Ordering;
    use std::collections::HashMap;

    use rstest::{fixture, rstest};

    use crate::specificity::Specificity as _;

    use super::*;

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
