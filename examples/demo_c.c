/**
 * ZirvTK C demo — builds a window with a label.
 *
 * Compile with (from the zirvtk directory):
 *   gcc -Iinclude -Ltarget/release -o demo_c examples/demo_c.c -lzirvtk
 */
#include "zirvtk.h"

int main(void) {
    ZtkApp *app = ztk_app_create();
    if (!app) return 1;

    ZtkWidget *label = ztk_label_create("Hello from C!");
    ZtkWidget *win   = ztk_window_create("C Demo", label);

    ztk_app_set_root(app, win);
    ztk_app_run(app);
    ztk_app_destroy(app);
    return 0;
}
