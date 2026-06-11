pub mod pmm;
pub mod vmm;

use limine::memmap::Entry;

pub fn init(entries: &[&Entry], hhdm_offset: u64) {
    pmm::init(entries, hhdm_offset);
    vmm::init();
}