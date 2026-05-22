//! ZirvTK demo — a simple window with a button and label.

use zirvtk::{App, Label, Panel, Widget, Window};
use alloc::boxed::Box;

fn main() {
    let mut app = App::new().expect("failed to connect to ZirvFlux");

    // Create a label wrapped in a panel
    let label = Label::new("Welcome to ZirvTK!");
    let panel = Panel::new(Some(Box::new(label)));

    // Create a window to hold it
    let window = Window::new("ZirvTK Demo", Some(Box::new(panel)));

    app.set_root(Box::new(window));
    app.run();
}
