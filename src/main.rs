mod algo;
mod circ;
mod format;
mod ui;

use ui::context::UiContext;

use imgui::*;

fn main_screen(ui: &Ui, data: &mut ()) {
    ui.text("Hello, World!");
}

fn main() {
    let ui_context = UiContext::new(main_screen, ());
    ui_context.run();
}
