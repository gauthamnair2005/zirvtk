# ZirvTK — MOSIX Desktop Toolkit & Widget Library

Rust GUI toolkit for **MOSIX** operating systems. Provides a complete desktop
compositor (ZirvTK Desktop) and widget library with C FFI for building
graphical applications on top of the ZirvFlux display framework.

Part of the [Zirvium](https://github.com/gauthamnair2005/zirvium) reference
MOSIX implementation.

## Features

### Desktop Compositor
- **Dark/neon/glass aesthetic** — futuristic UI with gradient backgrounds, glass panels, and glow effects
- **Real-time clock** — RTC-driven time with anti-aliased large-scale digits and formatted date
- **Taskbar** — bottom/top panel with launcher dot button and live time display
- **App Launcher** — grid-based app menu with SVG vector icons, hover effects, and pulsing animation
- **Particle system** — ambient floating particles with color cycling
- **Hardware cursor** — DisplayJet hardware cursor, no software cursor overhead
- **Tear-free rendering** — VBE page flipping via DisplayJet Y_OFFSET flip

### Widget System
- Widget trait with event propagation (MouseMove, MouseDown, MouseUp)
- AppLauncher — configurable app grid with styled icons
- TaskBar — panel with launcher button and clock
- FuturisticClock — anti-aliased large-scale clock display
- NeonButton, GlassPanel, GlowSlider, AnimatedToggle — decorative widgets
- Canvas — software rendering with scanline-optimized rounded rects

### Canvas Rendering
- Scanline-based `fill_round_rect` and `stroke_round_rect` (O(h) instead of O(w\*h))
- `fill_gradient_v` / `clear_gradient` — vertical gradient fills
- `draw_char_scaled` — bilinear-interpolated anti-aliased text
- `fill_circle` — integer edge-finding slice fill (no per-pixel sqrt)
- Alpha blending, glass panel rendering
- SVG-style vector path icon rendering

### Platform Support
- `no_std` + `alloc` compatible — runs in freestanding kernel environments
- C FFI (`zirvtk.h`) — link from C/C++ compositors
- DisplayJet / ZirvFlux backend

## Widgets

| Widget | Description |
|--------|-------------|
| `Widget` | Base trait: event handling, layout, draw |
| `AppLauncher` | Grid-based app menu with icons and hover effects |
| `TaskBar` | Desktop panel with launcher button and system clock |
| `FuturisticClock` | Anti-aliased large-format clock with date |
| `NeonButton` | Glowing neon-styled button |
| `GlassPanel` | Semi-transparent glass panel with border |
| `GlowSlider` | Slider control with glow handle |
| `AnimatedToggle` | Toggle switch with animated glow |

## Project Structure

```
src/
  lib.rs           Crate root, no_std entry point
  allocator.rs     Freestanding arena allocator
  app_launcher.rs  App launcher widget
  canvas.rs        Software renderer (scanline-optimized)
  c_api.rs         C FFI exports
  color.rs         Color types (RGBA, blend, gradient)
  desktop.rs       Desktop compositor
  ffi.rs           ZirvFlux FFI bindings
  font.rs          Bitmap font data (Inter)
  futuristic_clock.rs Clock widget with anti-aliased text
  fx.rs            Visual effects (glow, glass panel, scanline)
  glass_panel.rs   Glass panel widget
  glow_slider.rs   Glow slider widget
  neon_button.rs   Neon button widget
  obj3d.rs         3D object utilities
  particle.rs      Particle system
  platform.rs      Platform abstraction (DisplayBuffer, events)
  rawfb.rs         Raw framebuffer drawing + vector path icons
  rect.rs          Rectangle geometry
  taskbar.rs       Taskbar widget
  widget.rs        Widget trait + Event types
include/
  zirvtk.h         C header for FFI
```

## Build

```bash
# Standalone (with ZirvFlux dependency)
ZIRVFLUX_DIR=../zirvflux cargo build --release --no-default-features --features alloc

# As submodule in Zirvium
cd /path/to/zirvium/zirvtk
ZIRVFLUX_DIR=../zirvflux cargo build --release --no-default-features --features alloc
```

Builds `libzirvtk.a` and `libzirvtk.rlib`.

## Dependencies

- [zirvflux](https://github.com/gauthamnair2005/zirvflux) — Display framework
- `libm` — Math library (sqrt, sin, cos for effects and rendering)

## License

GPLv3
