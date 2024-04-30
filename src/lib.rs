use std::process::Command;

#[derive(Debug)]
pub enum OperatingSystem {
    Macos,
}

impl OperatingSystem {
    pub fn get_operating_system() -> OperatingSystem {
        if cfg!(target_os = "macos") {
            return OperatingSystem::Macos;
        }
        panic!("MacOS was not the compile target. Please add support for your OS.");
    }
}

#[derive(Debug)]
pub struct SystemInformation {
    pub os: OperatingSystem,
    pub low_battery: bool,
    pub fully_charged: bool,
    pub charge_state: u8,
}

impl SystemInformation {
    pub fn build() -> SystemInformation {
        SystemInformation {
            os: OperatingSystem::get_operating_system(),
            low_battery: false,
            fully_charged: false,
            charge_state: 100,
        }
    }

    pub fn profile_battery() -> String {
        String::from_utf8(
            Command::new("system_profiler")
                .arg("SPPowerDataType")
                .output()
                .expect("Failed to get system power information.")
                .stdout,
        )
        .unwrap()
    }
}
