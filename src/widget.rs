use crate::canvas::Canvas;
use crate::rect::Rect;

/// Event delivered to widgets.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    MouseMove { x: i32, y: i32 },
    MouseDown { x: i32, y: i32, button: u8 },
    MouseUp { x: i32, y: i32, button: u8 },
}

/// Return value from [`Widget::handle_event`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventResult {
    /// Event was consumed by this widget.
    Handled,
    /// Event was not consumed; pass to parent / next widget.
    Ignored,
    /// Widget requests a re-render.
    Redraw,
}

/// Base trait for all widgets.
pub trait Widget {
    /// Draw the widget onto the canvas at its current position.
    fn draw(&self, canvas: &mut Canvas);

    /// Handle an input event. The coordinates in the event are canvas-absolute.
    fn handle_event(&mut self, _ev: &Event, _rect: Rect) -> EventResult {
        EventResult::Ignored
    }

    /// The widget's bounding rectangle (canvas-absolute).
    fn rect(&self) -> Rect;

    /// Return the widget's desired minimum size.
    fn min_size(&self) -> (u32, u32);

    /// Set the widget's position.
    fn set_pos(&mut self, x: i32, y: i32);

    /// Set the widget's size.
    fn set_size(&mut self, w: u32, h: u32);
}
