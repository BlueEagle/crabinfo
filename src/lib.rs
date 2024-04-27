use std::process;

pub fn incomp_poop() {
    if !cfg!(target_os = "macos") {
        eprintln!("MacOS was not the compile target. Please add support for your OS.");
        process::exit(1);
    }
}
