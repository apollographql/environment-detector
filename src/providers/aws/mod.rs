use anyhow::Result;

/// Represents the supported AWS compute platforms.
#[derive(Clone, Debug)]
pub enum ComputePlatform {
    Ecs,
    Ec2,
    Fargate,
    Lambda,
}

impl ComputePlatform {
    /// Gets a list of environment variables used to detect a compute platform.
    // TODO: just return a Vec<String>?
    pub(crate) fn get_env_vars(&self) -> &'static [&'static str] {
        match self {
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
}

pub(crate) fn detect_compute_platform() -> Result<Option<ComputePlatform>> {
    Ok(None)
}
