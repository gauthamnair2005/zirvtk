# ZirvTK — MOSIX GUI Toolkit (Reference Widget Library)

Rust GUI toolkit for **MOSIX** operating systems. Provides a widget library
with C FFI for building graphical applications on top of the ZirvFlux display
framework.

Part of the [Zirvium](https://github.com/gauthamnair2005/zirvium) reference
MOSIX implementation. See the [MOSIX specification](https://github.com/gauthamnair2005/zirvworld)
for the full standard.

## Features

- Widget tree with event propagation and layout
- MediaTile — content tile with thumbnails, labels, gradients
- TileScroller — scrollable grid of MediaTiles with physics-based scrolling
- Canvas — software rendering surface with pixel, rect, text drawing
- Color, Rect, Font primitives
- App — application scaffold with platform abstraction
- C FFI (`zirvtk.h`) for linking from C/C++ compositors
- `no_std` compatible with custom allocator for freestanding environments

## Widgets

| Widget | Description |
|--------|-------------|
| `Widget` | Base trait: event handling, layout, draw |
| `Button` | Clickable button with label |
| `Label` | Static text label |
| `Panel` | Container for child widgets |
| `Window` | Top-level window with title bar |
| `Scroller` | Vertical/horizontal scroll container |
| `MediaTile` | Thumbnail + title + gradient tile |
| `TileScroller` | Grid-based media tile scroller |

## Project Structure

```
src/
  lib.rs           Crate root, no_std entry point
  allocator.rs     Freestanding arena allocator
  app.rs           Application scaffold
  button.rs        Button widget
  canvas.rs        Software renderer
  c_api.rs         C FFI exports
  color.rs         Color types
  ffi.rs           Internal FFI helpers
  font.rs          5x7 bitmap font data
  label.rs         Label widget
  panel.rs         Panel container widget
  platform.rs      Platform abstraction (DisplayBuffer, events)
  rect.rs          Rectangle geometry
  scroller.rs      Scroll container widget
  tile.rs          MediaTile widget
  widget.rs        Widget trait + Event types
  window.rs        Window widget
include/
  zirvtk.h         C header for FFI
```

## Build

```bash
cargo build --release
```

Builds `libzirvtk.a`, `libzirvtk.so`, and `libzirvtk.rlib`.

## Examples

- `demo_rust` — GUI demo in Rust
- `demo_c` — GUI demo in C (links via C FFI)

## Dependencies

- [zirvflux](https://github.com/gauthamnair2005/zirvflux) — Display framework
- [zirvlibc](https://github.com/gauthamnair2005/zirvlibc) — C library
