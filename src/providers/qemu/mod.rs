use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

pub struct Qemu;

impl Detector for Qemu {
    fn detect(&self, _smbios: &Smbios, _env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        None
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(&[], None)]
    fn test_qemu(#[case] input_vars: &[&str], #[case] expected_platform: Option<ComputePlatform>) {
        let env_vars: HashSet<&str> = input_vars.iter().fold(HashSet::new(), |mut vars, var| {
            vars.insert(var);
            vars
        });
        let actual_platform = Qemu.detect(&Smbios::from(("", "", "")), &env_vars);
        assert_eq!(expected_platform, actual_platform);
    }
}
