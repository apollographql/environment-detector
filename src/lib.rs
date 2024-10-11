use std::{collections::HashSet, ops::Deref};

mod env;
mod providers;
use providers::{aws, azure, gcp, kubernetes, nomad, qemu, ComputePlatform};
mod smbios;
use smbios::Smbios;

pub fn get_compute_platform() -> Option<ComputePlatform> {
    // Attempt to read SMBIOS data.
    let smbios = Smbios::new();

    let detectors = get_detectors();

    // Read current environment variables and match against those expected by supported platforms.
    let env_vars: HashSet<_> = detectors
        .iter()
        .flat_map(|detector| detector.env_vars())
        .filter(|var| env::hasenv(var))
        .map(Deref::deref)
        .collect();

    // Using SMBIOS and env var data, attempt to detect a platform.
    let compute_platform = detectors
        .iter()
        .filter_map(|detector| detector.detect(&smbios, &env_vars))
        .fold(None, |acc, new| match acc {
            // TODO: need to use is_superset_of here.
            Some(old) => Some(old),
            None => Some(new),
        });

    compute_platform
}

/// Trait for detecting the use of a compute platform.
pub(crate) trait Detector {
    /// Returns a [`ComputePlatform`] based on the given smbios data and environment variables.
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform>;

    /// Returns a list of environment variables used to detect a compute platform.
    fn env_vars(&self) -> &'static [&'static str];
}

fn get_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(aws::Ecs),
        Box::new(aws::Ec2),
        Box::new(aws::Fargate),
        Box::new(aws::Lambda),
        Box::new(azure::ContainerApps),
        Box::new(azure::ContainerAppsJob),
        Box::new(gcp::CloudRunGen1),
        Box::new(gcp::CloudRunGen2),
        Box::new(gcp::CloudRunJob),
        Box::new(kubernetes::Kubernetes),
        Box::new(nomad::Nomad),
        Box::new(qemu::Qemu),
    ]
}
