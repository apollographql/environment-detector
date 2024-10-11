use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const AZURE_VENDOR: &str = "microsoft";

/// Represents the Azure Container Apps platform.
pub struct ContainerApps;

impl Detector for ContainerApps {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_system_vendor(AZURE_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AzureContainerApps)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "CONTAINER_APP_ENV_DNS_SUFFIX",
            "CONTAINER_APP_HOSTNAME",
            "CONTAINER_APP_NAME",
            "CONTAINER_APP_PORT",
            "CONTAINER_APP_REPLICA_NAME",
            "CONTAINER_APP_REVISION",
        ]
    }
}

/// Represents the Azure Container Apps Job platform.
pub struct ContainerAppsJob;

impl Detector for ContainerAppsJob {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_system_vendor(AZURE_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AzureContainerAppsJob)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "CONTAINER_APP_JOB_EXECUTION_NAME",
            "CONTAINER_APP_JOB_NAME",
            "CONTAINER_APP_REPLICA_NAME",
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
    #[case::smbios_env_match(ContainerApps.env_vars(), Smbios::from(("", "", AZURE_VENDOR)), Some(ComputePlatform::AzureContainerApps))]
    #[case::smbios_no_match(&[], Smbios::from(("", "", AZURE_VENDOR)), None)]
    fn test_container_app(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = ContainerApps.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::from(("", "", "")), None)]
    #[case::smbios_env_match(ContainerAppsJob.env_vars(), Smbios::from(("", "", AZURE_VENDOR)), Some(ComputePlatform::AzureContainerAppsJob))]
    #[case::smbios_no_match(&[], Smbios::from(("", "", AZURE_VENDOR)), None)]
    fn test_container_app_job(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = ContainerAppsJob.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
