use crate::circ::sequence::Gate;

use imgui::*;

pub const GATE_SIZE: f32 = 50.0;
pub const GATE_PADDING: f32 = 10.0;
pub const GATE_TOTAL: f32 = GATE_SIZE + GATE_PADDING;

pub fn gate_on_grid(gate: Gate, x: &mut f32, y: f32, ql: usize, colour: [f32; 3], ui: &Ui) {
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
        Gate::RX { l, theta } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({})", theta));
                    });
                }
            }

            one_qubit_text_gate(ui, "RX", colour);
            *x += GATE_TOTAL;
        },
        Gate::RY { l, theta } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({})", theta));
                    });
                }
            }

            one_qubit_text_gate(ui, "RY", colour);
            *x += GATE_TOTAL;
        },
        Gate::RZ { l, theta } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({})", theta));
                    });
                }
            }

            one_qubit_text_gate(ui, "RZ", colour);
            *x += GATE_TOTAL;
        },
        Gate::P { l, theta } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({})", theta));
                    });
                }
            }

            one_qubit_text_gate(ui, "P", colour);
            *x += GATE_TOTAL;
        },
        Gate::U2 { l, phi, lambda } => {
            ui.set_cursor_pos([*x, y + GATE_TOTAL * l as f32]);

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({}, {})", phi, lambda));
                    });
                }
            }

            one_qubit_text_gate(ui, "U2", colour);
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
        Gate::SWAP { la, lb } => {
            {
                let dl = ui.get_window_draw_list();
                let (min, max) = min_max([la, lb]);
                ui.set_cursor_pos([*x + GATE_SIZE / 2.0, y + GATE_TOTAL * min as f32 + GATE_SIZE / 2.0]);
                let [x, y] = ui.cursor_screen_pos();
                dl.add_line(
                    [x, y],
                    [x, y + GATE_TOTAL * (max - min) as f32],
                    colour
                ).thickness(4.0).build();
            }

            ui.set_cursor_pos([*x, y + GATE_TOTAL * la as f32]);
            one_qubit_swap_gate(ui, colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * lb as f32]);
            one_qubit_swap_gate(ui, colour);

            *x += GATE_TOTAL;
        },
        Gate::CP { l, c, theta } => {
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

            {
                let [x, y] = ui.cursor_screen_pos();
                if ui.is_mouse_hovering_rect([x, y], [x + GATE_SIZE, y + GATE_SIZE]) {
                    ui.tooltip(|| {
                        ui.text(format!("({})", theta));
                    });
                }
            }

            one_qubit_text_gate(ui, "CP", colour);
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
        },
        Gate::CSWAP { la, lb, c } => {
            {
                let dl = ui.get_window_draw_list();
                let (min, max) = min_max([la, lb, c]);
                ui.set_cursor_pos([*x + GATE_SIZE / 2.0, y + GATE_TOTAL * min as f32 + GATE_SIZE / 2.0]);
                let [x, y] = ui.cursor_screen_pos();
                dl.add_line(
                    [x, y],
                    [x, y + GATE_TOTAL * (max - min) as f32],
                    colour
                ).thickness(4.0).build();
            }

            ui.set_cursor_pos([*x, y + GATE_TOTAL * la as f32]);
            one_qubit_swap_gate(ui, colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * lb as f32]);
            one_qubit_swap_gate(ui, colour);
            ui.set_cursor_pos([*x, y + GATE_TOTAL * c as f32]);
            one_qubit_control_gate(ui, colour);

            *x += GATE_TOTAL;
        }
    }
}

fn one_qubit_swap_gate(ui: &Ui, colour: [f32; 3]) {
    one_qubit_gate(ui, |dl, o, d| {
        dl.add_line([o.0 + d.0 * (1.0/4.0), o.1 + d.1 * (1.0/4.0)], [o.0 + d.0 * (3.0/4.0), o.1 + d.1 * (3.0/4.0)], colour)
            .thickness(4.0)
            .build();

        dl.add_line([o.0 + d.0 * (3.0/4.0), o.1 + d.1 * (1.0/4.0)], [o.0 + d.0 * (1.0/4.0), o.1 + d.1 * (3.0/4.0)], colour)
            .thickness(4.0)
            .build();
    });
}

fn one_qubit_control_gate(ui: &Ui, colour: [f32; 3]) {
    one_qubit_gate(ui, |dl, o, d| {
        dl.add_circle([o.0 + d.0 / 2.0, o.1 + d.1 / 2.0], 10.0, colour)
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
