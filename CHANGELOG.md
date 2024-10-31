# Changelog

All notable changes to the `environment-detector` library will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0

This is the initial release of `apollo-environment-detector`. It provides the following 2 functions
for detecting a Compute Environment:

```rust
/// Detect a single, most likely [`ComputeEnvironment`] above a certain weighted threshold.
pub fn detect_one(threshold: u16) -> Option<ComputeEnvironment>;

/// Detect potential [`ComputeEnvironment`]s above a certain weighted threshold.
///
/// Returns an ordered [`Vec`] with the highest weighted candidates first.
pub fn detect(threshold: u16) -> Vec<ComputeEnvironment>;
```
