use std::collections::HashSet;

use crate::ComputePlatform;

/// Gets a list of environment variables used to detect a compute platform.
fn get_env_vars() -> &'static [&'static str] {
    &[
        "KUBERNETES_PORT",
        "KUBERNETES_PORT_443_TCP",
        "KUBERNETES_PORT_443_TCP_ADDR",
        "KUBERNETES_PORT_443_TCP_PORT",
        "KUBERNETES_PORT_443_TCP_PROTO",
        "KUBERNETES_SERVICE_HOST",
        "KUBERNETES_SERVICE_PORT",
        "KUBERNETES_SERVICE_PORT_HTTPS",
    ]
}

pub(crate) fn detect_compute_platform(vars: HashSet<&str>) -> Option<ComputePlatform> {
    if vars.is_empty() {
        return None;
    }

    if vars.iter().all(|item| get_env_vars().contains(item)) {
        return Some(ComputePlatform::Kubernetes);
    }

    None
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::kubernetes(get_env_vars(), Some(ComputePlatform::Kubernetes))]
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
