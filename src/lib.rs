#![cfg_attr(target_os = "none", no_std)]
#![doc = "ZirvTK — Zirvium GUI Toolkit.\n\nA Rust widget library with C FFI for building graphical applications on the Zirvium OS compositor (DisplayJet / ZirvFlux)."]

extern crate alloc;

// On the freestanding target, provide a global allocator and panic handler.
#[cfg(target_os = "none")]
mod allocator;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub mod color;
pub mod font;
pub mod rect;

mod canvas;
pub use canvas::Canvas;

mod ffi;
mod platform;
pub use platform::{DisplayBuffer, DisplayInfo, MouseEvent, Platform};

pub mod widget;
pub use widget::{Event, EventResult, Widget};

pub mod button;
pub use button::Button;

pub mod label;
pub use label::Label;

pub mod panel;
pub use panel::Panel;

pub mod window;
pub use window::Window;

pub mod app;
pub use app::App;

// ── C FFI exports ──────────────────────────────────────────────────────────

mod c_api;
pub use c_api::*;
