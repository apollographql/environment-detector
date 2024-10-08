/// Returns `true` if the environment variable is set.
///
/// Note this function specifically uses libc in order to ensure we do _NOT_ hold the env var value
/// in memory, as this data should always be treated as secure regardless of the data.
#[cfg(unix)]
fn hasenv(name: &str) -> bool {
    let k = std::ffi::CString::new(name).unwrap();
    let v = unsafe { libc::getenv(k.as_ptr()) } as *const libc::c_char;
    !v.is_null()
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
