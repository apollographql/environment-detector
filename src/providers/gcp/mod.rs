use std::collections::HashSet;

/// Represents the differnent Cloud Run platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CloudRunPlatform {
    Gen1,
    Gen2,
    Job,
}

/// Represents the supported GCP compute platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePlatform {
    CloudRun(CloudRunPlatform),
}

/// Gets a list of environment variables used to detect a compute platform.
fn get_env_vars(platform: ComputePlatform) -> &'static [&'static str] {
    match platform {
        ComputePlatform::CloudRun(CloudRunPlatform::Gen1) => &[
            "K_REVISION",
            "K_SERVICE",
            "PORT",
            "K_CONFIGURATION",
            "CLOUD_RUN_TIMEOUT_SECONDS",
        ],
        ComputePlatform::CloudRun(CloudRunPlatform::Gen2) => &[
            "K_REVISION",
            "K_SERVICE",
            "PORT",
            "K_CONFIGURATION",
            "CLOUD_RUN_TIMEOUT_SECONDS",
        ],
        ComputePlatform::CloudRun(CloudRunPlatform::Job) => &[
            "CLOUD_RUN_EXECUTION",
            "CLOUD_RUN_JOB",
            "CLOUD_RUN_TASK_ATTEMPT",
            "CLOUD_RUN_TASK_COUNT",
            "CLOUD_RUN_TASK_INDEX",
        ],
    }
}

// TODO: difference between cloud run gen1 and gen2?
pub(crate) fn detect_compute_platform(vars: HashSet<&str>) -> Option<ComputePlatform> {
    if vars.is_empty() {
        return None;
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::CloudRun(CloudRunPlatform::Gen1)).contains(item))
    {
        return Some(ComputePlatform::CloudRun(CloudRunPlatform::Gen1));
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::CloudRun(CloudRunPlatform::Job)).contains(item))
    {
        return Some(ComputePlatform::CloudRun(CloudRunPlatform::Job));
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::cloud_run_gen1(
        get_env_vars(ComputePlatform::CloudRun(CloudRunPlatform::Gen1)),
        Some(ComputePlatform::CloudRun(CloudRunPlatform::Gen1))
    )]
    #[case::cloud_run_job(
        get_env_vars(ComputePlatform::CloudRun(CloudRunPlatform::Job)),
        Some(ComputePlatform::CloudRun(CloudRunPlatform::Job))
    )]
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
        let actual_platform = detect_compute_platform(env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
