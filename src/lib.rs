#![cfg_attr(target_os = "none", no_std)]

extern crate alloc;

#[cfg(target_os = "none")]
mod allocator;

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

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

pub mod tile;
pub use tile::MediaTile;

pub mod scroller;
pub use scroller::TileScroller;

pub mod app;
pub use app::App;

mod c_api;
pub use c_api::*;
