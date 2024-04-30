use crabinfo;

fn main() {
    let system_info = crabinfo::SystemInformation::build();
    println!("{:?}", system_info);
}
