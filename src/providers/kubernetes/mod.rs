use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

pub(crate) const KUBERNETES_ENV_VARS: &[&str] = &[
    "KUBERNETES_PORT",
    "KUBERNETES_PORT_443_TCP",
    "KUBERNETES_PORT_443_TCP_ADDR",
    "KUBERNETES_PORT_443_TCP_PORT",
    "KUBERNETES_PORT_443_TCP_PROTO",
    "KUBERNETES_SERVICE_HOST",
    "KUBERNETES_SERVICE_PORT",
    "KUBERNETES_SERVICE_PORT_HTTPS",
];

/// Represents the Kubernetes platform.
pub struct Kubernetes;

impl Detector for Kubernetes {
    fn detect(&self, _smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::Kubernetes)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        KUBERNETES_ENV_VARS
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(&[], None)]
    #[case::smbios_env_match(Kubernetes.env_vars(), Some(ComputePlatform::Kubernetes))]
    #[case::smbios_no_match(&[], None)]
    fn test_kubernetes(
        #[case] input_vars: &[&str],
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });

        let actual_platform = Kubernetes.detect(&Smbios::default(), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
