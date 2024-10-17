// AWS

pub const AWS_ECS: &[&str] = &[
    "AWS_EXECUTION_ENV",
    "ECS_AGENT_URI",
    "ECS_CONTAINER_METADATA_URI",
    "ECS_CONTAINER_METADATA_URI_V4",
];
pub const AWS_LAMBDA: &[&str] = &[
    "_AWS_XRAY_DAEMON_ADDRESS",
    "_AWS_XRAY_DAEMON_PORT",
    "_HANDLER",
    "AWS_ACCESS_KEY_ID",
    "AWS_DEFAULT_REGION",
    "AWS_EXECUTION_ENV",
    "AWS_LAMBDA_FUNCTION_MEMORY_SIZE",
    "AWS_LAMBDA_FUNCTION_NAME",
    "AWS_LAMBDA_FUNCTION_VERSION",
    "AWS_LAMBDA_INITIALIZATION_TYPE",
    "AWS_LAMBDA_LOG_GROUP_NAME",
    "AWS_LAMBDA_LOG_STREAM_NAME",
    "AWS_LAMBDA_RUNTIME_API",
    "AWS_REGION",
    "AWS_SECRET_ACCESS_KEY",
    "AWS_SESSION_TOKEN",
    "AWS_XRAY_CONTEXT_MISSING",
    "AWS_XRAY_DAEMON_ADDRESS",
    "LAMBDA_RUNTIME_DIR",
    "LAMBDA_TASK_ROOT",
];

// Azure

pub const AZURE_CONTAINER_APPS: &[&str] = &[
    "CONTAINER_APP_ENV_DNS_SUFFIX",
    "CONTAINER_APP_HOSTNAME",
    "CONTAINER_APP_NAME",
    "CONTAINER_APP_PORT",
    "CONTAINER_APP_REPLICA_NAME",
    "CONTAINER_APP_REVISION",
    "KUBERNETES_PORT",
    "KUBERNETES_PORT_443_TCP",
    "KUBERNETES_PORT_443_TCP_ADDR",
    "KUBERNETES_PORT_443_TCP_PORT",
    "KUBERNETES_PORT_443_TCP_PROTO",
    "KUBERNETES_SERVICE_HOST",
    "KUBERNETES_SERVICE_PORT",
    "KUBERNETES_SERVICE_PORT_HTTPS",
];
pub const AZURE_CONTAINER_APPS_JOB: &[&str] = &[
    "CONTAINER_APP_JOB_EXECUTION_NAME",
    "CONTAINER_APP_JOB_NAME",
    "CONTAINER_APP_REPLICA_NAME",
    "KUBERNETES_PORT",
    "KUBERNETES_PORT_443_TCP",
    "KUBERNETES_PORT_443_TCP_ADDR",
    "KUBERNETES_PORT_443_TCP_PORT",
    "KUBERNETES_PORT_443_TCP_PROTO",
    "KUBERNETES_SERVICE_HOST",
    "KUBERNETES_SERVICE_PORT",
    "KUBERNETES_SERVICE_PORT_HTTPS",
];
// TODO: check if this on Azure Service Fabric
// <https://learn.microsoft.com/en-us/azure/service-fabric/service-fabric-environment-variables-reference>
pub const AZURE_CONTAINER_INSTANCE: &[&str] = &[
    "Fabric_ApplicationName",
    "Fabric_CodePackageName",
    "Fabric_Id",
    "Fabric_NetworkingMode",
    "Fabric_NodeIPOrFQDN",
    "Fabric_ServiceDnsName",
    "Fabric_ServiceName",
];

// Google Cloud Platform

pub const GCP_CLOUD_RUN_SERVICE: &[&str] = &[
    "K_REVISION",
    "K_SERVICE",
    "PORT",
    "K_CONFIGURATION",
    "CLOUD_RUN_TIMEOUT_SECONDS",
];
pub const GCP_CLOUD_RUN_JOB: &[&str] = &[
    "CLOUD_RUN_EXECUTION",
    "CLOUD_RUN_JOB",
    "CLOUD_RUN_TASK_ATTEMPT",
    "CLOUD_RUN_TASK_COUNT",
    "CLOUD_RUN_TASK_INDEX",
];

// Generic sets

pub const EMPTY: &[&str] = &[];

pub const KUBERNETES: &[&str] = &[
    "KUBERNETES_PORT",
    "KUBERNETES_PORT_443_TCP",
    "KUBERNETES_PORT_443_TCP_ADDR",
    "KUBERNETES_PORT_443_TCP_PORT",
    "KUBERNETES_PORT_443_TCP_PROTO",
    "KUBERNETES_SERVICE_HOST",
    "KUBERNETES_SERVICE_PORT",
    "KUBERNETES_SERVICE_PORT_HTTPS",
];
pub const NOMAD: &[&str] = &[
    "    NOMAD_ALLOC_DIR",
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

/// Returns `true` if the environment variable is set.
///
/// Note this function specifically uses libc in order to ensure we do _NOT_ hold the env var value
/// in memory, as this data should always be treated as secure regardless of the data.
#[cfg(unix)]
pub fn hasenv(name: &str) -> bool {
    let k = std::ffi::CString::new(name).unwrap();
    let v = unsafe { libc::getenv(k.as_ptr()) } as *const libc::c_char;
    !v.is_null()
}

/// Returns `true` if the environment variable is set.
///
/// This is a failover method for non-UNIX systems, using the built-in `std::env::var_os` function.
#[cfg(not(unix))]
pub fn hasenv(name: &str) -> bool {
    std::env::var_os(name).is_some()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::hasenv;

    #[test]
    fn test_hasenv() {
        // Set an temporary env var for the current process.
        let var = "TEST_ENV_ENV_DETECTOR";
        env::set_var(var, "true");

        // Assert that temporary env vars do/don't exist.
        assert!(hasenv(var));
        assert!(!hasenv(&format!("{var}_NOT_SET")));
    }
}
