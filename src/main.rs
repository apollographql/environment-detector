fn main() {
    let env = apollo_environment_detector::detect();
    if let Some(env) = env {
        println!("{env:?}");
    }
}
