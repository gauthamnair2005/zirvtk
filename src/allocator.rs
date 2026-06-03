/// Minimal bump allocator for the ZirvTK freestanding environment.
///
/// Allocates from a fixed 4 MiB static pool. Freed memory is never reused
/// (bump-only). This is acceptable for the compositor app because the
/// widget tree is built once and lives for the lifetime of the process.

#[cfg(feature = "alloc")]
use core::alloc::{GlobalAlloc, Layout};
#[cfg(feature = "alloc")]
use core::ptr;
#[cfg(feature = "alloc")]
use core::sync::atomic::Ordering;
#[cfg(feature = "alloc")]
use core::sync::atomic::AtomicUsize;

#[cfg(feature = "alloc")]
const POOL_SIZE: usize = 16 * 1024 * 1024; // 16 MiB
#[cfg(feature = "alloc")]
static mut POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];

#[cfg(feature = "alloc")]
pub struct BumpAlloc {
    next: AtomicUsize,
}

#[cfg(feature = "alloc")]
impl BumpAlloc {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
        }
    }
}

#[cfg(feature = "alloc")]
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

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: BumpAlloc = BumpAlloc::new();
