use crate::circ::sequence::Gate;
use crate::algo::diff::Edit;
use crate::ui::circuit::lines;
use crate::ui::gate::*;

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
