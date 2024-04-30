use crabinfo;
use std::process::{Command, Output};

fn profile_battery() -> Output {
    Command::new("system_profiler")
        .arg("SPPowerDataType")
        .output()
        .expect("Failed to get system power information.")
}

fn main() {
    let system_info = crabinfo::SystemInformation::build();
    let battery_profile = profile_battery();
    let new_battery_profile = String::from_utf8(battery_profile.stdout).unwrap();
    println!("{:?}\n", new_battery_profile);
}
