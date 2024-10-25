//! # Compute Environment Detector
//!
//! This library provides two functions for easily detecting a [`ComputeEnvironment`] based on a
//! given weighted threshold.
//!
//! # Examples
//!
//! ```
//! use apollo_environment_detector::{detect, detect_one, MAX_INDIVIDUAL_WEIGHTING};
//!
//! // Attempt to detect multiple environments based on a weighting.
//! let compute_envs = detect(MAX_INDIVIDUAL_WEIGHTING);
//! println!("{:?}", compute_envs);
//!
//! // Attempt to detect a single environment based on a weighting.
//! let compute_env = detect_one(MAX_INDIVIDUAL_WEIGHTING);
//! println!("{:?}", compute_env);
//! ```

#![warn(missing_docs)]

use std::{cmp::Ordering, collections::HashSet, ops::Deref};

mod detector;
use detector::Detector;
mod env_vars;
mod environment;
pub use environment::{CloudProvider, ComputeEnvironment};
mod smbios;
use smbios::Smbios;
mod specificity;
use specificity::Specificity as _;

/// Represents the maximum weighting of all supported detectors (`2^15`).
///
/// This maximum weighting was chosen in order to have enough buffer compared
/// to avoid thresholding and overflows when using multiple detectors.
pub const MAX_TOTAL_WEIGHTING: u16 = 2 << 14;

/// Represents the maximum individual detector weighting.
///
/// There are currently 2 supported detectors:
/// - SMBIOS
/// - Environment Variables
pub const MAX_INDIVIDUAL_WEIGHTING: u16 = MAX_TOTAL_WEIGHTING / 2;

/// Detect a single, most likely [`ComputeEnvironment`] above a certain weighted threshold.
pub fn detect_one(threshold: u16) -> Option<ComputeEnvironment> {
    detect(threshold).first().copied()
}

/// Detect potential [`ComputeEnvironment`]s above a certain weighted threshold.
///
/// Returns an ordered [`Vec`] with the highest weighted candidates first.
pub fn detect(threshold: u16) -> Vec<ComputeEnvironment> {
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

fn detect_inner(
    detectors: Vec<Detector>,
    smbios: Smbios,
    env_vars: HashSet<&'static str>,
    threshold: u16,
) -> Vec<ComputeEnvironment> {
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
        .map(|(detector, _)| detector.environment)
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
            .iter()
            .map(Deref::deref)
            .collect();

        let result = detect_inner(detectors, smbios, env_vars, u16::MIN);

        assert_eq!(result.first(), Some(&environment));
    }

    #[rstest]
    fn test_missing_1_env_var(
        #[values(
            ComputeEnvironment::AwsEc2,
            ComputeEnvironment::AwsEcs,
            ComputeEnvironment::AwsLambda,
            ComputeEnvironment::AwsKubernetes,
            ComputeEnvironment::AwsNomad,
            // Accepted risk: these tests will fail if we remove one of the env var specific to
            // Azure Container Apps
            // ComputeEnvironment::AzureContainerApps,
            // ComputeEnvironment::AzureContainerAppsJob,
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
                result.first(),
                Some(&environment),
                "mismatch with {removed} removed"
            );
        }
    }

    #[rstest]
    fn test_missing_2_env_var(
        #[values(
            ComputeEnvironment::AwsEc2,
            ComputeEnvironment::AwsEcs,
            ComputeEnvironment::AwsLambda,
            ComputeEnvironment::AwsKubernetes,
            ComputeEnvironment::AwsNomad,
            // Accepted risk: these tests will fail if we remove two of the env vars specific to
            // Azure Container Apps
            // ComputeEnvironment::AzureContainerApps,
            // ComputeEnvironment::AzureContainerAppsJob,
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
            for j in 0..(env_vars.len() - 1) {
                let mut env_vars = env_vars.clone();
                let removed_1 = env_vars.remove(i);
                let removed_2 = env_vars.remove(j);
                let env_vars = env_vars.into_iter().collect();

                let result = detect_inner(detectors.clone(), smbios.clone(), env_vars, u16::MIN);

                assert_eq!(
                    result.first(),
                    Some(&environment),
                    "mismatch with {removed_1} and {removed_2} removed"
                );
            }
        }
    }
}
