use std::{cmp::Ordering, collections::HashSet, ops::Deref};

use detector::Detector;
use smbios::Smbios;
use specificity::Specificity as _;
use strum::{EnumIter, IntoEnumIterator as _};

mod detector;
mod env;
mod smbios;
mod specificity;

/// Represents the currently supported compute platforms.
#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumIter)]
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
    pub(crate) fn detector(&self) -> Detector {
        match self {
            ComputeEnvironment::AwsEc2 => Detector::new(*self, smbios::AWS, env::EMPTY),
            ComputeEnvironment::AwsEcs => Detector::new(*self, smbios::EMPTY, env::AWS_ECS),
            ComputeEnvironment::AwsLambda => Detector::new(*self, smbios::EMPTY, env::AWS_LAMBDA),
            ComputeEnvironment::AwsKubernetes => Detector::new(*self, smbios::AWS, env::KUBERNETES),
            ComputeEnvironment::AwsNomad => Detector::new(*self, smbios::AWS, env::NOMAD),
            ComputeEnvironment::AzureContainerApps => {
                Detector::new(*self, smbios::AZURE, env::AZURE_CONTAINER_APPS)
            }
            ComputeEnvironment::AzureContainerAppsJob => {
                Detector::new(*self, smbios::AZURE, env::AZURE_CONTAINER_APPS_JOB)
            }
            Self::AzureContainerInstance => {
                Detector::new(*self, smbios::EMPTY, env::AZURE_CONTAINER_INSTANCE)
            }
            ComputeEnvironment::AzureKubernetes => {
                Detector::new(*self, smbios::AZURE, env::KUBERNETES)
            }
            ComputeEnvironment::AzureVM => Detector::new(*self, smbios::AZURE, env::EMPTY),
            ComputeEnvironment::AzureNomad => Detector::new(*self, smbios::AZURE, env::NOMAD),
            ComputeEnvironment::GcpCloudRunGen1 => {
                Detector::new(*self, smbios::EMPTY, env::GCP_CLOUD_RUN_SERVICE)
            }
            ComputeEnvironment::GcpCloudRunGen2 => {
                Detector::new(*self, smbios::GCP, env::GCP_CLOUD_RUN_SERVICE)
            }
            ComputeEnvironment::GcpCloudRunJob => {
                Detector::new(*self, smbios::GCP, env::GCP_CLOUD_RUN_JOB)
            }
            ComputeEnvironment::GcpComputeEngine => Detector::new(*self, smbios::GCP, env::EMPTY),
            ComputeEnvironment::GcpKubernetes => Detector::new(*self, smbios::GCP, env::KUBERNETES),
            ComputeEnvironment::GcpNomad => Detector::new(*self, smbios::GCP, env::NOMAD),
            ComputeEnvironment::Kubernetes => Detector::new(*self, smbios::EMPTY, env::KUBERNETES),
            ComputeEnvironment::Nomad => Detector::new(*self, smbios::EMPTY, env::NOMAD),
            ComputeEnvironment::Qemu => Detector::new(*self, smbios::QEMU, env::EMPTY),
        }
    }
}

/// Attempts to calculate a compute environment based on SMBIOS data and environment variables present
/// at runtime.
pub fn detect() -> Option<ComputeEnvironment> {
    let detectors: Vec<_> = ComputeEnvironment::iter().map(|ce| ce.detector()).collect();

    // Read current environment variables and match against those expected by supported platforms.
    let env_vars: HashSet<_> = detectors
        .iter()
        .flat_map(|detector| detector.env_vars)
        .filter(|var| env::hasenv(var))
        .map(Deref::deref)
        .collect();

    // // Using SMBIOS and env var data, attempt to detect a platform.
    let smbios = Smbios::detect();
    let detector = detectors
        .into_iter()
        .filter(|detector| detector.detect(&smbios, &env_vars))
        .fold(None, |acc: Option<Detector>, new| match acc {
            None => Some(new),
            Some(old) => match old.specificity_cmp(&new) {
                Some(Ordering::Greater) | Some(Ordering::Equal) => Some(old),
                Some(Ordering::Less) => Some(new),
                None => {
                    // TODO: this shouldn't happen
                    Some(old)
                }
            },
        });

    detector.map(|detector| detector.environment)
}
