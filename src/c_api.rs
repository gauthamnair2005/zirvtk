//! C FFI — functions callable from C code that links against `libzirvtk.a`.
//!
//! Each function is `extern "C"` and follows the naming convention `ztk_*`.

use alloc::boxed::Box;
use core::ffi::{c_char, c_int};
use crate::app::App;
use crate::button::Button;
use crate::canvas::Canvas;
use crate::label::Label;
use crate::window::Window;
use crate::Widget;

/// Opaque handle types exposed to C.
pub enum ZtkApp {}
pub enum ZtkCanvas {}
pub enum ZtkWidget {}
pub enum ZtkButton {}
pub enum ZtkLabel {}
pub enum ZtkWindow {}

// ── App ───────────────────────────────────────────────────────────────────

/// Create a new ZirvTK application. Returns an opaque pointer or null on failure.
#[no_mangle]
pub extern "C" fn ztk_app_create() -> *mut ZtkApp {
    let app = match App::new() {
        Some(a) => a,
        None => return core::ptr::null_mut(),
    };
    Box::into_raw(Box::new(app)) as *mut ZtkApp
}

/// Run the event loop.
#[no_mangle]
pub extern "C" fn ztk_app_run(app: *mut ZtkApp) {
    let app = unsafe { &mut *(app as *mut App) };
    app.run();
}

/// Set the root widget.
#[no_mangle]
pub extern "C" fn ztk_app_set_root(app: *mut ZtkApp, widget: *mut ZtkWidget) {
    let app = unsafe { &mut *(app as *mut App) };
    let widget_box = unsafe { Box::from_raw(widget as *mut Box<dyn Widget>) };
    app.set_root(*widget_box);
}

/// Destroy the app (frees all owned widgets).
#[no_mangle]
pub extern "C" fn ztk_app_destroy(app: *mut ZtkApp) {
    if app.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(app as *mut App);
    }
}

// ── Button ────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ztk_button_create(text: *const c_char) -> *mut ZtkWidget {
    let s = unsafe { core::ffi::CStr::from_ptr(text) }
        .to_str()
        .unwrap_or("");
    let text_static: &'static str = Box::leak(Box::from(s));
    let btn = Button::new(text_static);
    let widget: Box<dyn Widget> = Box::new(btn);
    Box::into_raw(Box::new(widget)) as *mut ZtkWidget
}

#[no_mangle]
pub extern "C" fn ztk_label_create(text: *const c_char) -> *mut ZtkWidget {
    let s = unsafe { core::ffi::CStr::from_ptr(text) }
        .to_str()
        .unwrap_or("");
    let text_static: &'static str = Box::leak(Box::from(s));
    let label = Label::new(text_static);
    let widget: Box<dyn Widget> = Box::new(label);
    Box::into_raw(Box::new(widget)) as *mut ZtkWidget
}

#[no_mangle]
pub extern "C" fn ztk_window_create(
    title: *const c_char,
    content: *mut ZtkWidget,
) -> *mut ZtkWidget {
    let s = unsafe { core::ffi::CStr::from_ptr(title) }
        .to_str()
        .unwrap_or("Window");
    let title_static: &'static str = Box::leak(Box::from(s));
    let child: Option<Box<dyn Widget>> = if content.is_null() {
        None
    } else {
        let b = unsafe { Box::from_raw(content as *mut Box<dyn Widget>) };
        Some(*b)
    };
    let win = Window::new(title_static, child);
    let widget: Box<dyn Widget> = Box::new(win);
    Box::into_raw(Box::new(widget)) as *mut ZtkWidget
}

// ── Widget helpers ────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ztk_widget_set_pos(w: *mut ZtkWidget, x: c_int, y: c_int) {
    let widget = unsafe { &mut *(w as *mut Box<dyn Widget>) };
    (**widget).set_pos(x, y);
}

#[no_mangle]
pub extern "C" fn ztk_widget_set_size(w: *mut ZtkWidget, width: u32, height: u32) {
    let widget = unsafe { &mut *(w as *mut Box<dyn Widget>) };
    (**widget).set_size(width, height);
}

// ── Canvas (for custom drawing) ───────────────────────────────────────────

#[no_mangle]
pub extern "C" fn ztk_canvas_create(width: u32, height: u32) -> *mut ZtkCanvas {
    let canvas = Box::new(Canvas::new(width, height));
    Box::into_raw(canvas) as *mut ZtkCanvas
}

#[no_mangle]
pub extern "C" fn ztk_canvas_destroy(canvas: *mut ZtkCanvas) {
    if !canvas.is_null() {
        unsafe {
            let _ = Box::from_raw(canvas as *mut Canvas);
        }
    }
}
