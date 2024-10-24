use std::{cmp::Ordering, collections::HashSet, ops::Deref};

use crate::{
    smbios::{Smbios, SmbiosPattern},
    specificity::{OrderingExt, Specificity},
    ComputeEnvironment, MAX_INDIVIDUAL_WEIGHTING,
};

#[derive(Debug, Clone)]
pub struct Detector {
    pub environment: ComputeEnvironment,
    pub smbios: SmbiosPattern,
    pub env_vars: &'static [&'static str],
}

impl Detector {
    pub const fn new(
        environment: ComputeEnvironment,
        smbios: SmbiosPattern,
        env_vars: &'static [&'static str],
    ) -> Self {
        Self {
            environment,
            smbios,
            env_vars,
        }
    }

    /// Returns a score from 0-MAX_TOTAL_WEIGHTING representing the weight of the detected matches from
    /// SMBIOS information and environment variables.
    ///
    /// The score weighting works as follows:
    ///   - u16::MAX = 65535, which is 2^16-1
    ///   - the combined score goes from 0-2^15, therefore each component goes to 2^14 in order
    ///     to have enough buffer compared to 2^15 to avoid thresholding and overflows.
    pub fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&'static str>) -> u16 {
        let smbios_detect = self.smbios.detect(smbios);

        let env_vars_detect = if self.env_vars.is_empty() {
            // Half of the max individual weigh for a single detector to avoid giving too much weight
            // to empty matches.
            MAX_INDIVIDUAL_WEIGHTING / 2
        } else {
            (self.env_vars.iter().fold(0usize, |acc, env_var| {
                if env_vars.contains(env_var) {
                    acc + 1
                } else {
                    acc
                }
            }) * MAX_INDIVIDUAL_WEIGHTING as usize
                / self.env_vars.len()) as u16
        };

        smbios_detect + env_vars_detect
    }
}

impl Specificity for Detector {
    fn specificity_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_env_vars: HashSet<_> = self.env_vars.iter().map(Deref::deref).collect();
        let other_env_vars: HashSet<_> = other.env_vars.iter().map(Deref::deref).collect();

        self.smbios
            .specificity_cmp(&other.smbios)
            .merge_specificity(self_env_vars.specificity_cmp(&other_env_vars))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use rstest::rstest;

    use crate::{
        smbios::{self, Smbios, SmbiosPattern},
        ComputeEnvironment, MAX_INDIVIDUAL_WEIGHTING, MAX_TOTAL_WEIGHTING,
    };

    use super::Detector;

    #[rstest]
    #[case::match_smbios_env(smbios::TESTING, &["TESTING_ENV"], MAX_TOTAL_WEIGHTING)]
    #[case::match_smbios(smbios::TESTING, &[], MAX_INDIVIDUAL_WEIGHTING)]
    #[case::match_env(SmbiosPattern::new(), &["TESTING_ENV"], MAX_INDIVIDUAL_WEIGHTING + (MAX_INDIVIDUAL_WEIGHTING/2))]
    fn test_detector_detect(
        #[case] smbios_pattern: SmbiosPattern,
        #[case] env_vars: &'static [&'static str],
        #[case] expected: u16,
    ) {
        let smbios = Smbios::from(smbios_pattern.clone());

        let detected = Detector::new(
            ComputeEnvironment::Testing,
            smbios_pattern,
            &["TESTING_ENV"],
        )
        .detect(&smbios, &HashSet::from_iter(env_vars.iter().cloned()));

        assert_eq!(expected, detected);
    }
}
