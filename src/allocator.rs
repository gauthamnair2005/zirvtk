/// Minimal bump allocator for the ZirvTK freestanding environment.
///
/// Allocates from a fixed 4 MiB static pool. Freed memory is never reused
/// (bump-only). This is acceptable for the compositor app because the
/// widget tree is built once and lives for the lifetime of the process.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

const POOL_SIZE: usize = 4 * 1024 * 1024; // 4 MiB
static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];

pub struct BumpAlloc {
    next: AtomicUsize,
}

impl BumpAlloc {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
        }
    }
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        loop {
            let offset = self.next.load(Ordering::Acquire);
            let aligned = (offset + align - 1) & !(align - 1);
            if aligned + size > POOL_SIZE {
                return ptr::null_mut();
            }
            if self
                .next
                .compare_exchange_weak(offset, aligned + size, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return unsafe { POOL.as_mut_ptr().add(aligned) };
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // bump — no-op; memory is never freed
    }
}

#[global_allocator]
static ALLOCATOR: BumpAlloc = BumpAlloc::new();
