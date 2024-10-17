use apollo_environment_detector::detect;
use std::env::args;

fn main() {
    let threshold = args()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or_default();

    for (detected, detection) in detect(threshold) {
        println!("{detected:?}, {detection:.2}");
    }
}
