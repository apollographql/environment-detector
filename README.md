# apollo-environment-detector
[![CircleCI](https://circleci.com/gh/apollographql/environment-detector/tree/main.svg?style=shield)](https://circleci.com/gh/apollographql/environment-detector/tree/main)

This library provides two functions for easily detecting a [`ComputeEnvironment`](https://docs.rs/apollo-environment-detector/0.1/apollo-environment-detector/enum.ComputeEnvironment.html) based on a
given weighted threshold.

```
[dependencies]
apollo-environment-detector = "0.1.0"
```

## Usage
```rust
use apollo_environment_detector::{detect, MAX_INDIVIDUAL_WEIGHTING};

let compute_envs = detect(MAX_INDIVIDUAL_WEIGHTING);
println!("{:?}", compute_envs);
```

### Detectors

This library currently supports 2 detectors: [SMBIOS](https://en.wikipedia.org/wiki/System_Management_BIOS) and Environment Variables.

### SMBIOS

There are currently 3 data points read during detection on both Linux and Windows:
- `bios_vendor`
- `product_name`
- `sys_vendor`

### Threshold Weighting

A detection threshold is represented in the form of a `u16`, which has a max of `65535` (`2^16-1`) as defined in the [std lib](https://doc.rust-lang.org/std/primitive.u16.html#associatedconstant.MAX).

As we supported multiple detectors, the maximum returned total weighting is `2^15` in order to avoid thresholding and overflows when using multiple detectors. This is exposed as a constant [`MAX_TOTAL_WEIGHTING`](https://docs.rs/apollo-environment-detector/0.1/apollo-environment-detector/constant.MAX_TOTAL_WEIGHTING.html).
