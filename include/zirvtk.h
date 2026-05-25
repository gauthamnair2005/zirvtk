#ifndef ZIRVTK_H
#define ZIRVTK_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Opaque handles */
typedef struct ZtkApp     ZtkApp;
typedef struct ZtkWidget  ZtkWidget;
typedef struct ZtkCanvas  ZtkCanvas;

/* ── App lifecycle ──────────────────────────────────────────────────────── */
ZtkApp*    ztk_app_create(void);
void       ztk_app_run(ZtkApp *app);
void       ztk_app_set_root(ZtkApp *app, ZtkWidget *widget);
void       ztk_app_destroy(ZtkApp *app);

/* ── Widget constructors ────────────────────────────────────────────────── */
ZtkWidget* ztk_button_create(const char *text);
ZtkWidget* ztk_label_create(const char *text);
ZtkWidget* ztk_window_create(const char *title, ZtkWidget *content);
ZtkWidget* ztk_panel_create(ZtkWidget *child);

/* ── Widget helpers ─────────────────────────────────────────────────────── */
void       ztk_widget_set_pos(ZtkWidget *w, int x, int y);
void       ztk_widget_set_size(ZtkWidget *w, uint32_t width, uint32_t height);

/* ── Canvas (custom drawing) ────────────────────────────────────────────── */
ZtkCanvas* ztk_canvas_create(uint32_t width, uint32_t height);
void       ztk_canvas_destroy(ZtkCanvas *canvas);

/* ── Raw framebuffer drawing primitives (system-level UI toolkit) ───────── */
/*   All functions operate on a raw 32-bit ARGB pixel buffer.                */

/* Blend two colors with alpha (0-255). Returns blended ARGB pixel. */
uint32_t   ztk_fb_blend(uint32_t fg, uint32_t bg, uint8_t alpha);

/* Set a single pixel with bounds clipping. */
void       ztk_fb_set_pixel(uint32_t *fb, uint32_t w, uint32_t h,
                            int x, int y, uint32_t color);

/* Fill a rectangle. */
void       ztk_fb_fill_rect(uint32_t *fb, uint32_t w, uint32_t h,
                            int x, int y, uint32_t rw, uint32_t rh,
                            uint32_t color);

/* Fill a vertical gradient rectangle. */
void       ztk_fb_fill_gradient_v(uint32_t *fb, uint32_t w, uint32_t h,
                                  int x, int y, uint32_t gw, uint32_t gh,
                                  uint32_t top, uint32_t bot);

/* Draw a single 8x13 bitmap character. */
void       ztk_fb_draw_char(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                            int x, int y, uint8_t c, uint32_t color);

/* Draw a single character with configurable anti-aliasing (aa_level 0-255). */
void       ztk_fb_draw_char_aa(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                               int x, int y, uint8_t c, uint32_t color,
                               uint8_t aa_level);

/* Draw a null-terminated text string. */
void       ztk_fb_draw_text(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                            int x, int y, const uint8_t *text,
                            uint32_t color);

/* Draw a single character at integer scale. */
void       ztk_fb_draw_char_scaled(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                                   int x, int y, uint8_t c, uint32_t color,
                                   uint32_t scale);

/* Draw a null-terminated text string at integer scale. */
void       ztk_fb_draw_text_large(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                                  int x, int y, const uint8_t *text,
                                  uint32_t color, uint32_t scale);

/* Draw a horizontal line. */
void       ztk_fb_hline(uint32_t *fb, uint32_t w, uint32_t h,
                        int x, int y, uint32_t line_w, uint32_t color);

/* Fill a circle. */
void       ztk_fb_fill_circle(uint32_t *fb, uint32_t w, uint32_t h,
                              int cx, int cy, uint32_t r, uint32_t color);

/* Fill a rounded rectangle. */
void       ztk_fb_fill_round_rect(uint32_t *fb, uint32_t w, uint32_t h,
                                  int x, int y, uint32_t rw, uint32_t rh,
                                  uint32_t radius, uint32_t color);

/* Draw a line using Bresenham's algorithm. */
void       ztk_fb_draw_line(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                            int x0, int y0, int x1, int y1,
                            uint32_t color);

/* Draw a procedural icon (0-15). See icon_type values below. */
void       ztk_draw_icon(uint32_t *fb, uint32_t fb_w, uint32_t fb_h,
                         int cx, int cy, uint32_t size, uint32_t icon_type,
                         uint32_t color);

/* Icon types:
 *   0 = Terminal     1 = Calculator   2 = Files        3 = Clock
 *   4 = Settings     5 = Weather      6 = Photos       7 = Editor
 *   8 = Snake        9 = Pong        10 = Tetris      11 = Music
 *  12 = Mail        13 = Store       14 = Maps        15 = About
 */

#ifdef __cplusplus
}
#endif

#endif /* ZIRVTK_H */
