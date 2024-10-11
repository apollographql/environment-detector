use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const GCP_VENDOR: &str = "google";
const GCP_PRODUCT_NAME: &str = "google compute engine";

/// Represents the GCP Cloud Run Gen1 platform.
pub struct CloudRunGen1;

impl Detector for CloudRunGen1 {
    // We cannot detect smbios data in Cloud Run Gen1
    fn detect(&self, _smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
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
        if !smbios.is_system_vendor(GCP_VENDOR) && !smbios.is_product_name(GCP_PRODUCT_NAME) {
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
        if !smbios.is_system_vendor(GCP_VENDOR) && !smbios.is_product_name(GCP_PRODUCT_NAME) {
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
    #[case::no_match(&[], None)]
    #[case::smbios_env_match(CloudRunGen1.env_vars(), Some(ComputePlatform::GcpCloudRunGen1))]
    #[case::smbios_no_match(&[], None)]
    fn test_cloud_run_gen1(
        #[case] input_vars: &[&str],
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = CloudRunGen1.detect(&Smbios::default(), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::default(), None)]
    #[case::smbios_system_only_env_match(CloudRunGen2.env_vars(), Smbios::from(("", "", GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunGen2))]
    #[case::smbios_product_only_env_match(CloudRunGen2.env_vars(), Smbios::from(("", GCP_PRODUCT_NAME, "")), Some(ComputePlatform::GcpCloudRunGen2))]
    #[case::smbios_env_match(CloudRunGen2.env_vars(), Smbios::from(("", GCP_PRODUCT_NAME, GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunGen2))]
    #[case::smbios_system_only_no_match(&[], Smbios::from(("", "", GCP_VENDOR)), None)]
    #[case::smbios_product_only_no_match(&[], Smbios::from(("", GCP_PRODUCT_NAME, "")), None)]
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
    #[case::no_match(&[], Smbios::default(), None)]
    #[case::smbios_system_only_env_match(CloudRunJob.env_vars(), Smbios::from(("", "", GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunJob))]
    #[case::smbios_product_only_env_match(CloudRunJob.env_vars(), Smbios::from(("", GCP_PRODUCT_NAME, "")), Some(ComputePlatform::GcpCloudRunJob))]
    #[case::smbios_env_match(CloudRunJob.env_vars(), Smbios::from(("", GCP_PRODUCT_NAME, GCP_VENDOR)), Some(ComputePlatform::GcpCloudRunJob))]
    #[case::smbios_system_only_no_match(&[], Smbios::from(("", "", GCP_VENDOR)), None)]
    #[case::smbios_product_only_no_match(&[], Smbios::from(("", GCP_PRODUCT_NAME, "")), None)]
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
