use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const GCP_VENDOR: &str = "google";

/// Represents the GCP Cloud Run Gen1 platform.
pub struct CloudRunGen1;

impl Detector for CloudRunGen1 {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_system_vendor(GCP_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::GcpCloudRunGen1)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "K_REVISION",
            "K_SERVICE",
            "PORT",
            "K_CONFIGURATION",
            "CLOUD_RUN_TIMEOUT_SECONDS",
        ]
    }
}

/// Represents the GCP Cloud Run Gen2 platform.
pub struct CloudRunGen2;

impl Detector for CloudRunGen2 {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_system_vendor(GCP_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::GcpCloudRunGen2)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "K_REVISION",
            "K_SERVICE",
            "PORT",
            "K_CONFIGURATION",
            "CLOUD_RUN_TIMEOUT_SECONDS",
        ]
    }
}

/// Represents the GCP Cloud Run Job platform.
pub struct CloudRunJob;

impl Detector for CloudRunJob {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_system_vendor(GCP_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::GcpCloudRunJob)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "CLOUD_RUN_EXECUTION",
            "CLOUD_RUN_JOB",
            "CLOUD_RUN_TASK_ATTEMPT",
            "CLOUD_RUN_TASK_COUNT",
            "CLOUD_RUN_TASK_INDEX",
        ]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(&[], Smbios::from(("", "", "")), None)]
    #[case::smbios_env_match(CloudRunGen1.env_vars(), Smbios::from(("", "", GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunGen1))]
    #[case::smbios_no_match(&[], Smbios::from(("", "", GCP_VENDOR)), None)]
    fn test_cloud_run_gen1(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = CloudRunGen1.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::from(("", "", "")), None)]
    #[case::smbios_env_match(CloudRunGen2.env_vars(), Smbios::from(("", "", GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunGen2))]
    #[case::smbios_no_match(&[], Smbios::from(("", "", GCP_VENDOR)), None)]
    fn test_cloud_run_gen2(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = CloudRunGen2.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::from(("", "", "")), None)]
    #[case::smbios_env_match(CloudRunJob.env_vars(), Smbios::from(("", "", GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunJob))]
    #[case::smbios_no_match(&[], Smbios::from(("", "", GCP_VENDOR)), None)]
    fn test_cloud_run_job(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = CloudRunJob.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
