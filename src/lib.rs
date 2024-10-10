use std::{collections::HashSet, ops::Deref};

use anyhow::Result;

mod env;
mod providers;
use providers::*;
mod smbios;
use smbios::Smbios;

/// Represents the known platform details of a specific compute environment.
pub struct ComputeEnvironment {
    pub compute_platform: Option<ComputePlatform>,
}

pub fn get_compute_environment() -> Result<ComputeEnvironment> {
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
            Some(old) => Some(if old > new { old } else { new }),
            None => Some(new),
        });

    Ok(ComputeEnvironment { compute_platform })
}

pub trait Detector {
    /// Blah
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform>;

    /// Blah
    fn env_vars(&self) -> &'static [&'static str];
}

fn get_detectors() -> Vec<Box<dyn Detector>> {
    vec![
        Box::new(aws::Ecs),
        Box::new(aws::Ec2),
        Box::new(aws::Fargate),
        Box::new(aws::Lambda),
        Box::new(azure::ContainerApp),
        Box::new(azure::ContainerAppJob),
        Box::new(gcp::CloudRunGen1),
        Box::new(gcp::CloudRunGen2),
        Box::new(gcp::CloudRunJob),
        Box::new(kubernetes::Kubernetes),
        Box::new(nomad::Nomad),
        Box::new(qemu::Qemu),
    ]
}
