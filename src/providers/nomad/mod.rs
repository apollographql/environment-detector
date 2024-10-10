use std::collections::HashSet;

use crate::ComputePlatform;

/// Gets a list of environment variables used to detect a compute platform.
fn get_env_vars() -> &'static [&'static str] {
    &[
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
    ]
}

pub(crate) fn detect_compute_platform(vars: &HashSet<&str>) -> Option<ComputePlatform> {
    if vars.is_empty() {
        return None;
    }

    if vars.iter().all(|item| get_env_vars().contains(item)) {
        return Some(ComputePlatform::Nomad);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::nomad(get_env_vars(), Some(ComputePlatform::Nomad))]
    #[case::empty(&[], None)]
    #[case::no_match(&["ENV_A", "ENV_B"], None)]
    fn test_detect_compute_platform(
        #[case] input_vars: &[&str],
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = detect_compute_platform(&env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
