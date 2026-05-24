use zirvtk::{App, MediaTile, TileScroller, Widget, Color, Rect};
use alloc::boxed::Box;

fn main() {
    let mut app = App::new().expect("failed to connect to ZirvFlux");

    let mut scroller = TileScroller::new("ZIRV MEDIA CENTER");

    let tiles = [
        ("Pictures", "Your photo library", "[PIC]", Color::from_u32(0xFF2E8B57)),
        ("Videos", "Movies & clips", "[VID]", Color::from_u32(0xFF4169E1)),
        ("Music", "All your tracks", "[MSC]", Color::from_u32(0xFF8B008B)),
        ("TV", "Recorded shows", "[TV]",  Color::from_u32(0xFFB8860B)),
        ("Radio", "FM & online", "[RAD]", Color::from_u32(0xFFCD5C5C)),
        ("Tasks", "System tools", "[TSK]", Color::from_u32(0xFF2F4F4F)),
        ("Settings", "Preferences", "[SET]", Color::from_u32(0xFF555555)),
        ("Extras", "More apps", "[EXT]", Color::from_u32(0xFF556B2F)),
    ];

    for (title, sub, icon, color) in &tiles {
        scroller.add_tile(MediaTile::new(title, sub, icon, *color));
    }

    app.set_root(Box::new(scroller));
    app.run();
}
