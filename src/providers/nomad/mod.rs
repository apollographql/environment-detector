use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

pub(crate) const NOMAD_ENV_VARS: &[&str] = &[
    "NOMAD_ALLOC_DIR",
    "NOMAD_ALLOC_ID",
    "NOMAD_ALLOC_INDEX",
    "NOMAD_ALLOC_NAME",
    "NOMAD_CPU_CORES",
    "NOMAD_CPU_LIMIT",
    "NOMAD_DC",
    "NOMAD_GROUP_NAME",
    "NOMAD_JOB_ID",
    "NOMAD_JOB_NAME",
    "NOMAD_MEMORY_LIMIT",
    "NOMAD_NAMESPACE",
    "NOMAD_PARENT_CGROUP",
    "NOMAD_REGION",
    "NOMAD_SECRETS_DIR",
    "NOMAD_SHORT_ALLOC_ID",
    "NOMAD_TASK_DIR",
    "NOMAD_TASK_NAME",
];

/// Represents the HashiCorp Nomad platform.
pub struct Nomad;

impl Detector for Nomad {
    fn detect(&self, _smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::Nomad)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        NOMAD_ENV_VARS
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(&[], None)]
    #[case::smbios_env_match(Nomad.env_vars(), Some(ComputePlatform::Nomad))]
    #[case::smbios_no_match(&[],  None)]
    fn test_nomad(#[case] input_vars: &[&str], #[case] expected_platform: Option<ComputePlatform>) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Nomad.detect(&Smbios::from(("", "", "")), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
