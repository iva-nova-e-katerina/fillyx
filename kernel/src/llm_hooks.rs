use crate::arch::serial;

pub struct RhaiKernelEngine {
    pub initialized: bool,
}

static mut ENGINE: RhaiKernelEngine = RhaiKernelEngine { initialized: false };

pub fn init() {
    serial::write(b"LLM Hooks init... ");
    unsafe {
        ENGINE = RhaiKernelEngine { initialized: true };
    }
    serial::write(b"[OK]\n");
}

pub fn is_ready() -> bool {
    unsafe { ENGINE.initialized }
}

pub fn execute_script(script: &str) -> Result<(), &'static str> {
    if !is_ready() {
        return Err("LLM engine not initialized");
    }
    serial::write(b"LLM script queued: ");
    serial::write(script.as_bytes());
    serial::write(b"\n");
    Ok(())
}