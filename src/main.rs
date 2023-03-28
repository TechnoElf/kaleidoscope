mod algo;
mod circ;
mod format;
mod ui;

use std::fs;

use ui::context::UiContext;
use ui::circuit::circuit_window;
use ui::diff::diff_window;
use ui::file_picker::FilePicker;
use circ::sequence::{Circuit, Gate};
use format::openqasm;
use algo::diff::{Edit, EditGraph};

use imgui::*;

struct AppState {
    diff: Vec<Edit<Gate>>,
    diff_myers: Vec<Edit<Gate>>,
    circ_a: Circuit,
    circ_b: Circuit,
    file_picker_a: FilePicker,
    file_picker_b: FilePicker
}

fn main_screen(ui: &Ui, data: &mut AppState) {
    circuit_window(ui, "Circuit A", &data.circ_a);
    diff_window(ui, "A -> B (Dijkstra)", &data.diff, &data.circ_a, &data.circ_b);
    diff_window(ui, "A -> B (Myers)", &data.diff_myers, &data.circ_a, &data.circ_b);

    let mut file_picker_request_a = false;
    let mut file_picker_request_b = false;
    ui.main_menu_bar(|| {
        ui.menu("File", || {
            if ui.menu_item("Load Circuit A") {
                file_picker_request_a = true;
            }

            if ui.menu_item("Load Circuit B") {
                file_picker_request_b = true;
            }
        });
    });

    if file_picker_request_a {
        data.file_picker_a.open(ui);
    }

    if file_picker_request_b {
        data.file_picker_b.open(ui);
    }

    if let Some(f) = data.file_picker_a.update(ui) {
        let src = fs::read_to_string(f).unwrap();
        data.circ_a = openqasm::parse(src);
        let edit = EditGraph::new(data.circ_a.gates().clone(), data.circ_b.gates().clone());
        data.diff = edit.edit_script();
        data.diff_myers = edit.edit_script_myers();
    }

    if let Some(f) = data.file_picker_b.update(ui) {
        let src = fs::read_to_string(f).unwrap();
        data.circ_b = openqasm::parse(src);
        let edit = EditGraph::new(data.circ_a.gates().clone(), data.circ_b.gates().clone());
        data.diff = edit.edit_script();
        data.diff_myers = edit.edit_script_myers();
    }
}

fn main() {
    let circ_a = Circuit::new();
    let circ_b = Circuit::new();
    let edit = EditGraph::new(circ_a.gates().clone(), circ_b.gates().clone());

    let app_state = AppState {
        diff: edit.edit_script(),
        diff_myers: edit.edit_script_myers(),
        circ_a,
        circ_b,
        file_picker_a: FilePicker::new("fpa".to_string()),
        file_picker_b: FilePicker::new("fpb".to_string())
    };

    let ui_context = UiContext::new(main_screen, app_state);
    ui_context.run();
}
