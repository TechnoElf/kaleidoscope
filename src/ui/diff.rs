use crate::circ::sequence::Gate;
use crate::algo::diff::Edit;

use imgui::*;

const GATE_SIZE: f32 = 50.0;
const GATE_PADDING: f32 = 10.0;
const GATE_TOTAL: f32 = GATE_SIZE + GATE_PADDING;

pub fn diff_window(ui: &Ui, title: &str, diff: &Vec<Edit<Gate>>) {
    ui.window(title)
        .size([GATE_TOTAL * 4.0, GATE_TOTAL * 4.0], Condition::FirstUseEver)
        .horizontal_scrollbar(true)
        .build(|| {
            let (ql, cl) = (4, 4); // TODO
            let [mut x, y] = ui.cursor_pos();
            x += GATE_PADDING;
            let d = diff.len(); // TODO

            lines(ui, ql, cl, d);

            for edit in diff {
                match edit {
                    Edit::Insert(gate) => gate_on_grid(gate.clone(), &mut x, y, ql, [0.0, 1.0, 0.0], ui),
                    Edit::Remove(gate) => gate_on_grid(gate.clone(), &mut x, y, ql, [1.0, 0.0, 0.0], ui),
                    Edit::Keep(gate) => gate_on_grid(gate.clone(), &mut x, y, ql, [1.0, 1.0, 1.0], ui)
                }
            }
            
            ui.set_cursor_pos([x + GATE_PADDING, y + GATE_TOTAL * (ql + cl) as f32 + GATE_PADDING]);
        }); 
}

fn lines(ui: &Ui, ql: usize, cl: usize, d: usize) {
    for l in 0..ql {
        let dl = ui.get_window_draw_list();
        let [x, y] = ui.cursor_screen_pos();
        dl.add_line(
            [x, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL],
            [x + d as f32 * GATE_TOTAL + GATE_PADDING, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL],
            [1.0, 1.0, 1.0]
        ).thickness(2.0).build();
    }

    for l in 0..cl {
        let dl = ui.get_window_draw_list();
        let [x, y] = ui.cursor_screen_pos();
        let y = y + ql as f32 * GATE_TOTAL;
        dl.add_line(
            [x, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL - 2.0],
            [x + d as f32 * GATE_TOTAL + GATE_PADDING, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL - 2.0],
            [1.0, 1.0, 1.0]
        ).thickness(1.0).build();
        dl.add_line(
            [x, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL + 2.0],
            [x + d as f32 * GATE_TOTAL + GATE_PADDING, y + GATE_SIZE / 2.0 + l as f32 * GATE_TOTAL + 2.0],
            [1.0, 1.0, 1.0]
        ).thickness(1.0).build();
    }
}

fn gate_on_grid(gate: Gate, x: &mut f32, y: f32, ql: usize, colour: [f32; 3], ui: &Ui) {
    match gate {
        Gate::Measure { ql: mql, cl } => {
            {
                let dl = ui.get_window_draw_list();
                let (min, max) = min_max([mql, cl + ql]);
                ui.set_cursor_pos([*x + GATE_SIZE / 2.0, y + GATE_TOTAL * min as f32 + GATE_SIZE / 2.0]);
                let [x, y] = ui.cursor_screen_pos();
                dl.add_line(
                    [x, y],
                    [x, y + GATE_TOTAL * (max - min) as f32],
                    colour
                ).thickness(4.0).build();
            }

            ui.set_cursor_pos([*x, y + GATE_TOTAL * mql as f32]);
            one_qubit_text_gate(ui, "M", colour);

            *x += GATE_TOTAL;
        },
        Gate::H { l } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "H", colour);
            *x += GATE_TOTAL;
        },
        Gate::X { l } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "X", colour);
            *x += GATE_TOTAL;
        },
        Gate::Y { l } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "Y", colour);
            *x += GATE_TOTAL;
        },
        Gate::Z { l } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "Z", colour);
            *x += GATE_TOTAL;
        },
        Gate::SX { l } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "SX", colour);
            *x += GATE_TOTAL;
        },
        Gate::CX { l, c } => {
            {
                let dl = ui.get_window_draw_list();
                let (min, max) = min_max([l, c]);
                ui.set_cursor_pos([*x + GATE_SIZE / 2.0, y + GATE_TOTAL * min as f32 + GATE_SIZE / 2.0]);
                let [x, y] = ui.cursor_screen_pos();
                dl.add_line(
                    [x, y],
                    [x, y + GATE_TOTAL * (max - min) as f32],
                    colour
                ).thickness(4.0).build();
            }

            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "X", colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * c as f32]);
            one_qubit_control_gate(ui, colour);

            *x += GATE_TOTAL;
        },
        Gate::CCX { l, c0, c1 } => {
            {
                let dl = ui.get_window_draw_list();
                let (min, max) = min_max([l, c0, c1]);
                ui.set_cursor_pos([*x + GATE_SIZE / 2.0, y + GATE_TOTAL * min as f32 + GATE_SIZE / 2.0]);
                let [x, y] = ui.cursor_screen_pos();
                dl.add_line(
                    [x, y],
                    [x, y + GATE_TOTAL * (max - min) as f32 - GATE_PADDING],
                    colour
                ).thickness(4.0).build();
            }

            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);
            one_qubit_text_gate(ui, "X", colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * c0 as f32]);
            one_qubit_control_gate(ui, colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * c1 as f32]);
            one_qubit_control_gate(ui, colour);
            
            *x += GATE_TOTAL;
        }
    }
}

fn one_qubit_control_gate(ui: &Ui, colour: [f32; 3]) {
    one_qubit_gate(ui, |dl, o, d| {
        dl.add_circle([o.0 + d.0  / 2.0, o.1 + d.1 / 2.0], 10.0, colour)
            .filled(true)
            .build();
    });
}

fn one_qubit_text_gate(ui: &Ui, text: &str, colour: [f32; 3]) {
    ui.set_window_font_scale(1.5);
    one_qubit_rect_gate(ui, |dl, o, d| {
        let [w, h] = ui.calc_text_size(text);
        dl.add_text([o.0 + (d.0 - w) / 2.0, o.1 + (d.1 - h) / 2.0], colour, text);
    }, colour);
    ui.set_window_font_scale(1.0);
}

fn one_qubit_rect_gate(ui: &Ui, content: impl Fn(&DrawListMut, (f32, f32), (f32, f32)), colour: [f32; 3]) {
    one_qubit_gate(ui, |dl, o, d| {
        dl.add_rect([o.0, o.1], [o.0 + d.0, o.1 + d.1], [0.0, 0.0, 0.0])
            .thickness(2.0)
            .filled(true)
            .build();

        content(&dl, (o.0 + GATE_PADDING / 2.0, o.1 + GATE_PADDING / 2.0), (d.0 - GATE_PADDING, d.1 - GATE_PADDING));

        dl.add_rect([o.0, o.1], [o.0 + d.0, o.1 + d.1], colour)
            .thickness(2.0)
            .build();
    });
}

fn one_qubit_gate(ui: &Ui, content: impl Fn(&DrawListMut, (f32, f32), (f32, f32))) {
    let dl = ui.get_window_draw_list();
    let [x, y] = ui.cursor_screen_pos();

    content(&dl, (x, y), (GATE_SIZE, GATE_SIZE));

    let [x, y] = ui.cursor_pos();
    ui.set_cursor_pos([x + GATE_TOTAL, y]);
}

fn min_max<const N: usize>(values: [usize; N]) -> (usize, usize) {
    (*values.iter().min().unwrap(), *values.iter().max().unwrap())
}
