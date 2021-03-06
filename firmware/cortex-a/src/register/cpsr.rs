//! Current Program Status Register

/// Reads the CPSR register
pub fn read() -> u32 {
    let cpsr: u32;
    unsafe { asm!("mrs $0, cpsr" : "=r"(cpsr)) }
    cpsr
}
