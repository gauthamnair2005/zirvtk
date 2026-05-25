#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "std"))]
mod allocator;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

pub mod color;
pub mod font;
pub mod rect;

#[cfg(feature = "alloc")]
mod canvas;
#[cfg(feature = "alloc")]
pub use canvas::Canvas;

mod ffi;
#[cfg(any(feature = "std", feature = "alloc"))]
mod platform;
#[cfg(any(feature = "std", feature = "alloc"))]
pub use platform::{DisplayBuffer, DisplayInfo, MouseEvent, Platform};

#[cfg(feature = "alloc")]
pub mod widget;
#[cfg(feature = "alloc")]
pub use widget::{Event, EventResult, Widget};

#[cfg(feature = "alloc")]
pub mod tile;
#[cfg(feature = "alloc")]
pub use tile::MediaTile;

#[cfg(feature = "alloc")]
pub mod scroller;
#[cfg(feature = "alloc")]
pub use scroller::TileScroller;

#[cfg(feature = "alloc")]
pub mod app;
#[cfg(feature = "alloc")]
pub use app::App;

#[cfg(feature = "alloc")]
mod button;
#[cfg(feature = "alloc")]
mod label;
#[cfg(feature = "alloc")]
mod panel;
#[cfg(feature = "alloc")]
mod window;

pub mod rawfb;

#[cfg(feature = "alloc")]
pub mod obj3d;

#[cfg(feature = "alloc")]
pub use obj3d::*;

mod c_api;
#[cfg(feature = "alloc")]
pub use c_api::*;
