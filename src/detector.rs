use crate::{
    smbios::{Smbios, SmbiosPattern},
    specificity::{OrderingExt, Specificity},
    ComputeEnvironment,
};
use std::{cmp::Ordering, collections::HashSet, ops::Deref};

pub struct Detector {
    pub environment: ComputeEnvironment,
    smbios: SmbiosPattern,
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
    pub fn detect(&self, smbios: &Smbios, env_vars: &HashSet<&'static str>) -> bool {
        self.smbios.matches(smbios)
            && self
                .env_vars
                .iter()
                .all(|env_var| env_vars.contains(env_var))
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
