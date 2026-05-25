//! ZirvFlux FFI — raw `extern "C"` declarations for the C ZirvFlux library.
//!
//! These are the lowest-level bindings used by the safe [`super::Platform`] wrapper.

#![allow(non_camel_case_types, dead_code)]

use core::ffi::c_void;

pub const ZF_CONNECTOR_NAME: usize = 64;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct zf_display_info {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u8,
    pub connected: i32,
    pub connector_name: [u8; ZF_CONNECTOR_NAME],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct zf_buffer {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub bpp: u8,
    pub data: *mut c_void,
    pub data_size: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct zf_mouse_event {
    pub dx: i32,
    pub dy: i32,
    pub buttons: u8,
}

extern "C" {
    pub fn zf_connect() -> i32;
    pub fn zf_disconnect();
    pub fn zf_get_info(info: *mut zf_display_info) -> i32;
    pub fn zf_create_buffer(width: u32, height: u32, buf: *mut zf_buffer) -> i32;
    pub fn zf_destroy_buffer(buf: *mut zf_buffer);
    pub fn zf_write_buffer(buf: *const zf_buffer, data: *const u8, size: usize) -> i32;
    pub fn zf_present(buf: *const zf_buffer) -> i32;
    pub fn zf_present_region(buf: *const zf_buffer, x: u32, y: u32, w: u32, h: u32) -> i32;
    pub fn zf_set_perf_mode(mode: i32) -> i32;
    pub fn zf_set_cursor(x: i32, y: i32) -> i32;
    pub fn zf_read_mouse(ev: *mut zf_mouse_event) -> i32;
    pub fn zf_reboot();
}
