//! ZirvTK demo — a simple window with a button and label.

use zirvtk::{App, Button, Label, Widget, Window};
use alloc::boxed::Box;

fn main() {
    let mut app = App::new().expect("failed to connect to ZirvFlux");

    // Create a button
    let mut button = Button::new("Click me!");

    // Create a label
    let label = Label::new("Welcome to ZirvTK!");

    // Create a window to hold them
    let content = Box::new(label);
    let window = Window::new("ZirvTK Demo", Some(content));

    app.set_root(Box::new(window));
    app.run();
}
