use crabinfo;
use std::process::{Command, Output};
use std::str;

fn profile_battery() -> Output {
    Command::new("system_profiler")
        .arg("SPPowerDataType")
        .output()
        .expect("Failed to get system power information.")
}

fn main() {
    let system_info = crabinfo::SystemInformation::build();
    let battery_profile = profile_battery();
    // let new_battery_profile = battery_profile.stdout.split("\n\n");
    let new_battery_profile = str::from_utf8(&battery_profile.stdout).unwrap();
    println!("{:?}\n", system_info);
    println!("{:?}", new_battery_profile);
}
