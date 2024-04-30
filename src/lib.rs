pub enum System {
    Macos,
}

pub fn incomp_poop() -> Result<System, &'static str> {
    if cfg!(target_os = "macos") {
        return Ok(System::Macos);
    }
    Err("MacOS was not the compile target. Please add support for your OS.")
}
