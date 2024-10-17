use crate::{
    smbios::{Smbios, SmbiosPattern},
    specificity::{OrderingExt, Specificity},
    ComputeEnvironment,
};
use std::{cmp::Ordering, collections::HashSet, ops::Deref};

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

    /// Returns `true` if this detector matches the SMBIOS information and set of environment
    /// variables
    ///
    /// This returns a score from 0 to 32768
    pub fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&'static str>) -> u16 {
        let smbios_detect = self.smbios.detect(smbios);

        let env_vars_detect = if self.env_vars.len() == 0 {
            8192
        } else {
            (self.env_vars.iter().fold(0usize, |acc, env_var| {
                if env_vars.contains(env_var) {
                    acc + 1
                } else {
                    acc
                }
            }) * 16384
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
