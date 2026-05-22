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

/* ── Widget helpers ─────────────────────────────────────────────────────── */
void       ztk_widget_set_pos(ZtkWidget *w, int x, int y);
void       ztk_widget_set_size(ZtkWidget *w, uint32_t width, uint32_t height);

/* ── Canvas (custom drawing) ────────────────────────────────────────────── */
ZtkCanvas* ztk_canvas_create(uint32_t width, uint32_t height);
void       ztk_canvas_destroy(ZtkCanvas *canvas);

#ifdef __cplusplus
}
#endif

#endif /* ZIRVTK_H */
