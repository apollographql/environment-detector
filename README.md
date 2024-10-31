# apollo-environment-detector
[![CircleCI](https://circleci.com/gh/apollographql/environment-detector/tree/main.svg?style=shield)](https://circleci.com/gh/apollographql/environment-detector/tree/main)

This library provides two functions for easily detecting a [`ComputeEnvironment`](https://docs.rs/apollo-environment-detector/0.1.0/apollo_environment_detector/enum.ComputeEnvironment.html) based on a
given weighted threshold.

```
[dependencies]
apollo-environment-detector = "0.1"
```

## Usage
```rust
use apollo_environment_detector::{detect, detect_one, MAX_INDIVIDUAL_WEIGHTING};

// Attempt to detect multiple environments based on a weighting.
let compute_envs = detect(MAX_INDIVIDUAL_WEIGHTING);
println!("{:?}", compute_envs);

// Attempt to detect a single environment based on a weighting.
let compute_env = detect_one(MAX_INDIVIDUAL_WEIGHTING);
println!("{:?}", compute_env);
```

### Detectors
The following environments are able to be detected:
- Amazon Elastic Compute Cloud (EC2)
- Amazon Elastic Container Service (ECS)
- AWS Lambda
- Kubernetes on AWS
- Nomad on AWS
- Azure Containers Apps
- Azure Container Apps Job
- Azure Container Instance
- Kubernetes on Azure
- Azure VM
- Nomad on Azure
- Google Cloud Run (Gen1)
- Google Cloud Run (Gen2)
- Google Cloud Run (Job)
- Google Compute Engine
- Kubernetes on Google Cloud
- Nomad on Google Cloud
- Kubernetes
- Nomad
- QEMU

This library currently supports 2 detection methods: [SMBIOS](https://en.wikipedia.org/wiki/System_Management_BIOS) and Environment Variables.

#### SMBIOS
There are currently 3 data points read during detection on both Linux and Windows:
- `bios_vendor`
- `product_name`
- `sys_vendor`

### Threshold Weighting
A detection threshold is represented in the form of a `u16`, which has a max of `65535` (`2^16-1`) as defined in `u16::MAX`.

As we supported multiple detectors, the maximum returned total weighting is `2^15` in order to avoid thresholding and overflows when using multiple detectors. This is exposed as a constant [`MAX_TOTAL_WEIGHTING`](https://docs.rs/apollo-environment-detector/0.1.0/apollo_environment_detector/constant.MAX_TOTAL_WEIGHTING.html).

