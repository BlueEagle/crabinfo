use crabinfo;
use crabinfo::SystemInformation;

fn main() {
    let system_info = SystemInformation::build();
    let battery_profile = SystemInformation::profile_battery();
    println!("{:?}\n", system_info);
    println!("{:?}\n", battery_profile);
}
