use std::collections::HashSet;

/// Represents the supported Azure compute platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePlatform {
    ContainerApp,
    ContainerAppJob,
}

/// Gets a list of environment variables used to detect a compute platform.
fn get_env_vars(platform: ComputePlatform) -> &'static [&'static str] {
    match platform {
        ComputePlatform::ContainerApp => &[
            "CONTAINER_APP_ENV_DNS_SUFFIX",
            "CONTAINER_APP_HOSTNAME",
            "CONTAINER_APP_NAME",
            "CONTAINER_APP_PORT",
            "CONTAINER_APP_REPLICA_NAME",
            "CONTAINER_APP_REVISION",
        ],
        ComputePlatform::ContainerAppJob => &[
            "CONTAINER_APP_JOB_EXECUTION_NAME",
            "CONTAINER_APP_JOB_NAME",
            "CONTAINER_APP_REPLICA_NAME",
        ],
    }
}

pub(crate) fn detect_compute_platform(vars: HashSet<&str>) -> Option<ComputePlatform> {
    if vars.is_empty() {
        return None;
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::ContainerApp).contains(item))
    {
        return Some(ComputePlatform::ContainerApp);
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::ContainerAppJob).contains(item))
    {
        return Some(ComputePlatform::ContainerAppJob);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::container_app(
        get_env_vars(ComputePlatform::ContainerApp),
        Some(ComputePlatform::ContainerApp)
    )]
    #[case::container_app_job(
        get_env_vars(ComputePlatform::ContainerAppJob),
        Some(ComputePlatform::ContainerAppJob)
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
