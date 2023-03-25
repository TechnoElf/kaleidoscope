use crate::circ::sequence::Circuit;
use crate::ui::gate::*;

use imgui::*;

pub fn circuit_window(ui: &Ui, title: &str, circ: &Circuit) {
    ui.window(title)
        .size([GATE_TOTAL * 4.0, GATE_TOTAL * 4.0], Condition::FirstUseEver)
        .horizontal_scrollbar(true)
        .build(|| {
            let (ql, cl) = circ.lines();
            let [mut x, y] = ui.cursor_pos();
            x += GATE_PADDING;
            let d = circ.gates().len();

            lines(ui, ql, cl, d);

            for gate in circ.gates() {
                gate_on_grid(gate.clone(), &mut x, y, ql, [1.0, 1.0, 1.0], ui)
            }
            
            ui.set_cursor_pos([x + GATE_PADDING, y + GATE_TOTAL * (ql + cl) as f32 + GATE_PADDING]);
        }); 
}

pub fn lines(ui: &Ui, ql: usize, cl: usize, d: usize) {
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
