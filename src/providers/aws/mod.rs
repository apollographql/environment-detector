use std::collections::HashSet;

/// Represents the supported AWS compute platforms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComputePlatform {
    Ecs,
    Ec2,
    Fargate,
    Lambda,
}

/// Gets a list of environment variables used to detect a compute platform.
fn get_env_vars(platform: ComputePlatform) -> &'static [&'static str] {
    match platform {
        ComputePlatform::Ecs => &[
            "AWS_EXECUTION_ENV",
            "ECS_AGENT_URI",
            "ECS_CONTAINER_METADATA_URI",
            "ECS_CONTAINER_METADATA_URI_V4",
        ],
        ComputePlatform::Ec2 => &[],
        ComputePlatform::Fargate => &[],
        ComputePlatform::Lambda => &[
            "AWS_LAMBDA_FUNCTION_MEMORY_SIZE",
            "AWS_LAMBDA_FUNCTION_NAME",
            "AWS_LAMBDA_FUNCTION_VERSION",
            "AWS_LAMBDA_INITIALIZATION_TYPE",
            "AWS_LAMBDA_LOG_GROUP_NAME",
            "AWS_LAMBDA_LOG_STREAM_NAME",
            "AWS_LAMBDA_RUNTIME_API",
        ],
    }
}

// TODO: EC2/Fargate detection.
pub(crate) fn detect_compute_platform(vars: HashSet<&str>) -> Option<ComputePlatform> {
    if vars.is_empty() {
        return None;
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::Ecs).contains(item))
    {
        return Some(ComputePlatform::Ecs);
    }

    if vars
        .iter()
        .all(|item| get_env_vars(ComputePlatform::Lambda).contains(item))
    {
        return Some(ComputePlatform::Lambda);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::ecs(get_env_vars(ComputePlatform::Ecs), Some(ComputePlatform::Ecs))]
    #[case::lambda(get_env_vars(ComputePlatform::Lambda), Some(ComputePlatform::Lambda))]
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
