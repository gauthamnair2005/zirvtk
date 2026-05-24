//! Platform abstraction — safe wrappers over the raw ZirvFlux FFI calls.

use crate::ffi;

/// Info returned by the display server.
#[derive(Clone, Debug)]
pub struct DisplayInfo {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u8,
    pub connected: bool,
}

/// A GPU-side or kernel-side display buffer.
pub struct DisplayBuffer {
    pub(crate) raw: ffi::zf_buffer,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
}

/// A relative mouse event.
#[derive(Clone, Copy, Debug)]
pub struct MouseEvent {
    pub dx: i32,
    pub dy: i32,
    pub buttons: u8,
}

/// High-level display connection handle.
pub struct Platform {
    info: DisplayInfo,
}

impl Platform {
    /// Connect to the ZirvFlux compositor. Returns `None` on failure.
    pub fn connect() -> Option<Self> {
        if unsafe { ffi::zf_connect() } != 0 {
            return None;
        }
        let mut raw = ffi::zf_display_info {
            width: 0,
            height: 0,
            stride: 0,
            bpp: 0,
            connected: 0,
            connector_name: [0; ffi::ZF_CONNECTOR_NAME],
        };
        if unsafe { ffi::zf_get_info(&mut raw) } != 0 {
            unsafe { ffi::zf_disconnect() };
            return None;
        }
        if raw.connected == 0 {
            unsafe { ffi::zf_disconnect() };
            return None;
        }
        let info = DisplayInfo {
            width: raw.width,
            height: raw.height,
            stride: raw.stride,
            bpp: raw.bpp,
            connected: raw.connected != 0,
        };
        Some(Self { info })
    }

    pub fn info(&self) -> &DisplayInfo {
        &self.info
    }

    /// Create an off-screen buffer.
    pub fn create_buffer(&self, width: u32, height: u32) -> Option<DisplayBuffer> {
        let mut raw = ffi::zf_buffer {
            id: 0,
            width: 0,
            height: 0,
            stride: 0,
            bpp: 0,
            data: core::ptr::null_mut(),
            data_size: 0,
        };
        if unsafe { ffi::zf_create_buffer(width, height, &mut raw) } != 0 {
            return None;
        }
        Some(DisplayBuffer {
            width: raw.width,
            height: raw.height,
            stride: raw.stride,
            raw,
        })
    }

    /// Write pixel data to a buffer.
    pub fn write_buffer(&self, buf: &DisplayBuffer, data: &[u8]) -> bool {
        unsafe { ffi::zf_write_buffer(&buf.raw, data.as_ptr(), data.len()) == 0 }
    }

    /// Present (flip) a buffer to the display.
    pub fn present(&self, buf: &DisplayBuffer) -> bool {
        unsafe { ffi::zf_present(&buf.raw) == 0 }
    }

    /// Present a region of a buffer.
    pub fn present_region(&self, buf: &DisplayBuffer, x: u32, y: u32, w: u32, h: u32) -> bool {
        unsafe { ffi::zf_present_region(&buf.raw, x, y, w, h) == 0 }
    }

    /// Set performance mode (0 = standard/secure, 1 = high performance).
    pub fn set_perf_mode(mode: bool) -> bool {
        unsafe { ffi::zf_set_perf_mode(if mode { 1 } else { 0 }) == 0 }
    }

    /// Set cursor position.
    pub fn set_cursor(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::zf_set_cursor(x, y) == 0 }
    }

    /// Read the next mouse event (non-blocking). Returns `None` if none available.
    pub fn read_mouse(&self) -> Option<MouseEvent> {
        let mut raw = ffi::zf_mouse_event {
            dx: 0,
            dy: 0,
            buttons: 0,
        };
        if unsafe { ffi::zf_read_mouse(&mut raw) } != 0 {
            return None;
        }
        Some(MouseEvent {
            dx: raw.dx,
            dy: raw.dy,
            buttons: raw.buttons,
        })
    }

    /// Reboot the system.
    pub fn reboot(&self) {
        unsafe { ffi::zf_reboot() }
    }

    /// Suppress debug output.
    pub fn suppress_dbg(&self) {
        unsafe { ffi::zf_suppress_dbg() }
    }

    /// Destroy a buffer.
    pub fn destroy_buffer(&self, buf: &mut DisplayBuffer) {
        unsafe { ffi::zf_destroy_buffer(&mut buf.raw) }
    }
}

impl Drop for Platform {
    fn drop(&mut self) {
        unsafe { ffi::zf_disconnect() }
    }
}
