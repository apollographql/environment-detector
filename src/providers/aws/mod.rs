use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const AWS_VENDOR: &str = "amazon ec2";

/// Represents the AWS ECS platform.
pub struct Ecs;

impl Detector for Ecs {
    // We cannot detect smbios data in ECS.
    fn detect(&self, _smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
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
    fn detect(&self, smbios: &Smbios, _env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_bios_vendor(AWS_VENDOR) && !smbios.is_system_vendor(AWS_VENDOR) {
            return None;
        }

        Some(ComputePlatform::AwsEc2)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Represents the AWS Fargate platform.
pub struct Fargate;

impl Detector for Fargate {
    fn detect(&self, smbios: &Smbios, _env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        if !smbios.is_bios_vendor(AWS_VENDOR) && !smbios.is_system_vendor(AWS_VENDOR) {
            return None;
        }

        Some(ComputePlatform::AwsFargate)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

/// Represents the AWS Lambda platform.
pub struct Lambda;

impl Detector for Lambda {
    fn detect(&self, _smbios: &Smbios, env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
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
    #[case::no_match(&[], None)]
    #[case::env_match(Ecs.env_vars(), Some(ComputePlatform::AwsEcs))]
    fn test_ecs(#[case] input_vars: &[&str], #[case] expected_platform: Option<ComputePlatform>) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Ecs.detect(&Smbios::default(), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::default(), None)]
    #[case::smbios_bios_match(Ecs.env_vars(), Smbios::from((AWS_VENDOR, "", "")), Some(ComputePlatform::AwsEc2))]
    #[case::smbios_system_match(Ecs.env_vars(), Smbios::from(("", "", AWS_VENDOR)), Some(ComputePlatform::AwsEc2))]
    #[case::smbios_bios_system_match(Ecs.env_vars(), Smbios::from((AWS_VENDOR, "", AWS_VENDOR)), Some(ComputePlatform::AwsEc2))]
    fn test_ec2(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Ec2.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], Smbios::default(), None)]
    #[case::smbios_bios_match(Ecs.env_vars(), Smbios::from((AWS_VENDOR, "", "")), Some(ComputePlatform::AwsFargate))]
    #[case::smbios_system_match(Ecs.env_vars(), Smbios::from(("", "", AWS_VENDOR)), Some(ComputePlatform::AwsFargate))]
    #[case::smbios_bios_system_match(Ecs.env_vars(), Smbios::from((AWS_VENDOR, "", AWS_VENDOR)), Some(ComputePlatform::AwsFargate))]
    fn test_fargate(
        #[case] input_vars: &[&str],
        #[case] smbios: Smbios,
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Fargate.detect(&smbios, &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }

    #[rstest]
    #[case::no_match(&[], None)]
    #[case::env_match(Lambda.env_vars(), Some(ComputePlatform::AwsLambda))]
    fn test_lambda(
        #[case] input_vars: &[&str],
        #[case] expected_platform: Option<ComputePlatform>,
    ) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Lambda.detect(&Smbios::default(), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
