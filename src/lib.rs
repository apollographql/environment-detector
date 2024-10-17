use std::{cmp::Ordering, collections::HashSet, ops::Deref};

use detector::Detector;
pub use environment::{CloudProvider, ComputeEnvironment};
use smbios::Smbios;
use specificity::Specificity as _;

mod detector;
mod env_vars;
mod environment;
mod smbios;
mod specificity;

/// Detect potential [`ComputeEnvironment`]s above a certain match threshold
///
/// This return an ordered [`Vec`], with the most likely candidates first.
pub fn detect(threshold: u16) -> Vec<(ComputeEnvironment, u16)> {
    let detectors: Vec<_> = ComputeEnvironment::iter().map(|ce| ce.detector()).collect();

    // Read current environment variables
    let env_vars: HashSet<_> = detectors
        .iter()
        .flat_map(|detector| detector.env_vars)
        .filter(|var| env_vars::hasenv(var))
        .map(Deref::deref)
        .collect();

    // Read SMBIOS data
    let smbios = Smbios::detect();

    // Run detectors against env vars and SMBIOS data
    detect_inner(detectors, smbios, env_vars, threshold)
}

/// Detect
fn detect_inner(
    detectors: Vec<Detector>,
    smbios: Smbios,
    env_vars: HashSet<&'static str>,
    threshold: u16,
) -> Vec<(ComputeEnvironment, u16)> {
    let mut detectors: Vec<_> = detectors
        .into_iter()
        .filter_map(|detector| {
            let score = detector.detect(&smbios, &env_vars);
            if score >= threshold {
                Some((detector, score))
            } else {
                None
            }
        })
        .collect();

    detectors.sort_by(|(left, left_score), (right, right_score)| {
        match Ord::cmp(left_score, right_score) {
            Ordering::Equal => left
                .specificity_cmp(right)
                .unwrap_or(Ordering::Equal)
                .reverse(),
            o => o.reverse(),
        }
    });
    detectors
        .into_iter()
        .map(|(detector, score)| (detector.environment, score))
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn detectors() -> Vec<Detector> {
        ComputeEnvironment::iter().map(|ce| ce.detector()).collect()
    }

    #[rstest]
    fn test_complete(
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
        environment: ComputeEnvironment,
        detectors: Vec<Detector>,
    ) {
        let smbios: Smbios = environment.detector().smbios.clone().into();
        let env_vars: HashSet<_> = environment
            .detector()
            .env_vars
            .into_iter()
            .map(Deref::deref)
            .collect();

        let result = detect_inner(detectors, smbios, env_vars, u16::MIN);

        assert_eq!(result.first().map(|(ce, _)| ce), Some(&environment));
    }

    #[rstest]
    fn test_partial_1_env_vars(
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
        environment: ComputeEnvironment,
        detectors: Vec<Detector>,
    ) {
        let smbios: Smbios = environment.detector().smbios.clone().into();
        let env_vars = environment.detector().env_vars.to_vec();

        for i in 0..(env_vars.len()) {
            let mut env_vars = env_vars.clone();
            let removed = env_vars.remove(i);
            let env_vars = env_vars.into_iter().collect();

            let result = detect_inner(detectors.clone(), smbios.clone(), env_vars, u16::MIN);

            assert_eq!(
                result.first().map(|(ce, _)| ce),
                Some(&environment),
                "mismatch with {removed} removed"
            );
        }
    }
}
