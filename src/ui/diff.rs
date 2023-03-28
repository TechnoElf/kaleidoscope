use std::cmp;

use crate::circ::sequence::{Gate, Circuit};
use crate::algo::diff::Edit;
use crate::ui::circuit::lines;
use crate::ui::gate::*;

use imgui::*;

const GATE_SIZE: f32 = 50.0;
const GATE_PADDING: f32 = 10.0;
const GATE_TOTAL: f32 = GATE_SIZE + GATE_PADDING;

pub fn diff_window(ui: &Ui, title: &str, diff: &Vec<Edit<Gate>>, circ_a: &Circuit, circ_b: &Circuit) {
    ui.window(title)
        .size([GATE_TOTAL * 4.0, GATE_TOTAL * 4.0], Condition::FirstUseEver)
        .horizontal_scrollbar(true)
        .build(|| {
            let (qla, cla) = circ_a.lines();
            let (qlb, clb) = circ_b.lines();
            let (ql, cl) = (cmp::max(qla, qlb), cmp::max(cla, clb));
            let [mut x, y] = ui.cursor_pos();
            x += GATE_PADDING;
            let d = diff.len();

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
