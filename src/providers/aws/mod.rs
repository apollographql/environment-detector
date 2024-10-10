use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const AWS_VENDOR: &str = "amazon";

/// Represents the AWS ECS platform.
pub struct Ecs;

impl Detector for Ecs {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_vendor(AWS_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AwsEcs)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "AWS_EXECUTION_ENV",
            "ECS_AGENT_URI",
            "ECS_CONTAINER_METADATA_URI",
            "ECS_CONTAINER_METADATA_URI_V4",
        ]
    }
}

/// Represents the AWS EC2 platform.
pub struct Ec2;

impl Detector for Ec2 {
    // TODO: better smbios matching.
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_vendor(AWS_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AwsEc2)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Represents the AWS Fargate platform.
pub struct Fargate;

impl Detector for Fargate {
    // TODO: better smbios matching.
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_vendor(AWS_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AwsFargate)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Represents the AWS Lambda platform.
pub struct Lambda;

impl Detector for Lambda {
    fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_vendor(AWS_VENDOR) {
            return None;
        }

        if env_vars.is_empty() {
            return None;
        }

        env_vars
            .iter()
            .all(|var| self.env_vars().contains(var))
            .then_some(ComputePlatform::AwsLambda)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[
            "AWS_LAMBDA_FUNCTION_MEMORY_SIZE",
            "AWS_LAMBDA_FUNCTION_NAME",
            "AWS_LAMBDA_FUNCTION_VERSION",
            "AWS_LAMBDA_INITIALIZATION_TYPE",
            "AWS_LAMBDA_LOG_GROUP_NAME",
            "AWS_LAMBDA_LOG_STREAM_NAME",
            "AWS_LAMBDA_RUNTIME_API",
        ]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(&[], Smbios {dmi_sys_vendor: None}, None)]
    #[case::smbios_env_match(Ecs.env_vars(), Smbios {dmi_sys_vendor: Some(AWS_VENDOR.to_string())}, Some(ComputePlatform::AwsEcs))]
    #[case::smbios_no_match(&[], Smbios {dmi_sys_vendor: Some(AWS_VENDOR.to_string())}, None)]
    fn test_ecs(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Ecs.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios {dmi_sys_vendor: None}, None)]
    #[case::smbios_env_match(Lambda.env_vars(), Smbios {dmi_sys_vendor: Some(AWS_VENDOR.to_string())}, Some(ComputePlatform::AwsLambda))]
    #[case::smbios_no_match(&[], Smbios {dmi_sys_vendor: Some(AWS_VENDOR.to_string())}, None)]
    fn test_lambda(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Lambda.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
