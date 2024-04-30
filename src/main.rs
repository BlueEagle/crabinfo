use crabinfo;
use std::process::Command;

fn main() {
    crabinfo::incomp_poop().expect("There was an issue getting system information.");
}
