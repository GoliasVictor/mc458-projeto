/// https://www.ntietz.com/blog/rust-hashmap-overhead/
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOC: AtomicUsize = AtomicUsize::new(0);
static DEALLOC: AtomicUsize = AtomicUsize::new(0);

pub struct TrackingAllocator;

pub fn record_alloc(layout: Layout) {
    ALLOC.fetch_add(layout.size(), Ordering::SeqCst);
}

pub fn record_dealloc(layout: Layout) {
    DEALLOC.fetch_add(layout.size(), Ordering::SeqCst);
}

pub fn reset() {
    ALLOC.store(0, Ordering::SeqCst);
    DEALLOC.store(0, Ordering::SeqCst);
}

pub fn stats() -> Stats {
    let alloc = ALLOC.load(Ordering::SeqCst);
    let dealloc = DEALLOC.load(Ordering::SeqCst);
    let diff = (alloc as isize) - (dealloc as isize);

    Stats {
        alloc,
        dealloc,
        diff,
    }
}

pub struct Stats {
    pub alloc: usize,
    pub dealloc: usize,
    pub diff: isize, 
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		unsafe {
			let p = System.alloc(layout);
			record_alloc(layout);
			p
		}
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		unsafe {
			record_dealloc(layout); 
			System.dealloc(ptr, layout);
		}
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;