use std::collections::HashSet;

use crate::{ComputePlatform, Detector, Smbios};

const QEMU_SYSTEM_VENDOR: &str = "qemu";

pub struct Qemu;

impl Detector for Qemu {
    fn detect(&self, smbios: &Smbios, _env_vars: &HashSet<&str>) -> Option<ComputePlatform> {
        smbios
            .is_system_vendor(QEMU_SYSTEM_VENDOR)
            .then_some(ComputePlatform::Qemu)
    }

    fn env_vars(&self) -> &'static [&'static str] {
        &[]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::no_match(Smbios::default(), None)]
    #[case::system_vendor_match(Smbios::from(("", "", QEMU_SYSTEM_VENDOR)), Some(ComputePlatform::Qemu))]
    fn test_qemu(#[case] smbios: Smbios, #[case] expected_platform: Option<ComputePlatform>) {
        let actual_platform = Qemu.detect(&smbios, &HashSet::new());
        assert_eq!(expected_platform, actual_platform);
    }
}
