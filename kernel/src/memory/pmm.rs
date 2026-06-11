use core::sync::atomic::{AtomicU64, Ordering};

use limine::memmap::{Entry, MEMMAP_USABLE};

pub const PAGE_SIZE: u64 = 4096;

static mut BITMAP: *mut u8 = core::ptr::null_mut();
static TOTAL_PAGES: AtomicU64 = AtomicU64::new(0);
static FREE_PAGES: AtomicU64 = AtomicU64::new(0);

pub fn init(entries: &[&Entry], hhdm_offset: u64) {
    let mut last_addr: u64 = 0;

    for entry in entries {
        let end = entry.base + entry.length;
        if end > last_addr {
            last_addr = end;
        }
    }

    let total_pages = last_addr / PAGE_SIZE;
    let bitmap_size = (total_pages + 7) / 8;

    let bitmap_addr = {
        let mut addr = 0;
        for entry in entries {
            if entry.type_ == MEMMAP_USABLE && entry.length >= bitmap_size {
                addr = entry.base + hhdm_offset;
                break;
            }
        }
        addr
    };

    unsafe {
        BITMAP = bitmap_addr as *mut u8;
    }

    TOTAL_PAGES.store(total_pages, Ordering::SeqCst);

    for i in 0..bitmap_size {
        unsafe {
            *BITMAP.add(i as usize) = 0xFF;
        }
    }

    let mut free_count = 0u64;
    for entry in entries {
        if entry.type_ == MEMMAP_USABLE {
            let base_page = entry.base / PAGE_SIZE;
            let page_count = entry.length / PAGE_SIZE;
            for p in 0..page_count {
                let page_idx = base_page + p;
                if page_idx < total_pages {
                    unsafe {
                        let byte_idx = (page_idx / 8) as usize;
                        let bit = (page_idx % 8) as u8;
                        if byte_idx < bitmap_size as usize {
                            *BITMAP.add(byte_idx) &= !(1 << bit);
                            free_count += 1;
                        }
                    }
                }
            }
        }
    }

    FREE_PAGES.store(free_count, Ordering::SeqCst);
}

pub fn allocate_page() -> Option<u64> {
    let total = TOTAL_PAGES.load(Ordering::SeqCst);
    for i in 0..total {
        unsafe {
            let byte_idx = (i / 8) as usize;
            let bit = (i % 8) as u8;
            let byte = *BITMAP.add(byte_idx) & (1 << bit);
            if byte == 0 {
                *BITMAP.add(byte_idx) |= 1 << bit;
                FREE_PAGES.fetch_sub(1, Ordering::SeqCst);
                return Some(i * PAGE_SIZE);
            }
        }
    }
    None
}

pub fn free_page(addr: u64) {
    let page = addr / PAGE_SIZE;
    unsafe {
        let byte_idx = (page / 8) as usize;
        let bit = (page % 8) as u8;
        *BITMAP.add(byte_idx) &= !(1 << bit);
    }
    FREE_PAGES.fetch_add(1, Ordering::SeqCst);
}

pub fn free_count() -> u64 {
    FREE_PAGES.load(Ordering::SeqCst)
}