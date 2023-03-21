mod algo;
mod circ;
mod format;
mod ui;

use ui::context::UiContext;
use ui::circuit::circuit_window;
use ui::diff::diff_window;
use circ::sequence::{Circuit, Gate};
use format::openqasm;
use algo::diff::{Edit, EditGraph};

use imgui::*;

struct AppState {
    diff: Vec<Edit<Gate>>,
    circ_a: Circuit,
    circ_b: Circuit
}

fn main_screen(ui: &Ui, data: &mut AppState) {
    circuit_window(ui, "Circuit A", &data.circ_a);
    diff_window(ui, "A -> B", &data.diff);
}

fn main() {
    let circ_a = openqasm::parse(SRCA.to_string());
    let circ_b = openqasm::parse(SRCB.to_string());
    let edit = EditGraph::new(circ_a.gates().clone(), circ_b.gates().clone());

    let app_state = AppState {
        diff: edit.edit_script(),
        circ_a,
        circ_b
    };

    let ui_context = UiContext::new(main_screen, app_state);
    ui_context.run();
}

const SRCA: &str = "
    // Benchmark was created by MQT Bench on 2022-12-15
    // For more information about MQT Bench, please visit https://www.cda.cit.tum.de/mqtbench/
    // MQT Bench version: 0.2.2
    // Qiskit version: {'qiskit-terra': '0.22.3', 'qiskit-aer': '0.11.1', 'qiskit-ignis': '0.7.0', 'qiskit-ibmq-provider': '0.19.2', 'qiskit': '0.39.3', 'qiskit-nature': '0.5.1', 'qiskit-finance': '0.3.4', 'qiskit-optimization': '0.4.0', 'qiskit-machine-learning': '0.5.0'}

    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[3];
    qreg flag[1];
    creg meas[4];
    h q[0];
    h q[1];
    h q[2];
    x flag[0];
    cp(pi/4) q[2],flag[0];
    cx q[2],q[1];
    cp(-pi/4) q[1],flag[0];
    cx q[2],q[1];
    cp(pi/4) q[1],flag[0];
    cx q[1],q[0];
    cp(-pi/4) q[0],flag[0];
    cx q[2],q[0];
    cp(pi/4) q[0],flag[0];
    cx q[1],q[0];
    cp(-pi/4) q[0],flag[0];
    u2(0,0) q[1];
    cx q[2],q[0];
    cp(pi/4) q[0],flag[0];
    u2(0,0) q[0];
    u1(-pi) q[2];
    ccx q[0],q[1],q[2];
    u2(-pi,-pi) q[0];
    u2(-pi,-pi) q[1];
    u1(-pi) q[2];
    cp(pi/4) q[2],flag[0];
    cx q[2],q[1];
    cp(-pi/4) q[1],flag[0];
    cx q[2],q[1];
    cp(pi/4) q[1],flag[0];
    cx q[1],q[0];
    cp(-pi/4) q[0],flag[0];
    cx q[2],q[0];
    cp(pi/4) q[0],flag[0];
    cx q[1],q[0];
    cp(-pi/4) q[0],flag[0];
    u2(0,0) q[1];
    cx q[2],q[0];
    cp(pi/4) q[0],flag[0];
    u2(0,0) q[0];
    u1(-pi) q[2];
    ccx q[0],q[1],q[2];
    u2(-pi,-pi) q[0];
    u2(-pi,-pi) q[1];
    u1(-pi) q[2];
    barrier q[0],q[1],q[2],flag[0];
    measure q[0] -> meas[0];
    measure q[1] -> meas[1];
    measure q[2] -> meas[2];
    measure flag[0] -> meas[3];
";

const SRCB: &str = "
    // Benchmark was created by MQT Bench on 2022-12-14
    // For more information about MQT Bench, please visit https://www.cda.cit.tum.de/mqtbench/
    // MQT Bench version: 0.2.2
    // Qiskit version: {'qiskit-terra': '0.22.3', 'qiskit-aer': '0.11.1', 'qiskit-ignis': None, 'qiskit-ibmq-provider': '0.19.2', 'qiskit': '0.39.3', 'qiskit-nature': '0.5.2', 'qiskit-finance': '0.3.4', 'qiskit-optimization': '0.4.0', 'qiskit-machine-learning': '0.5.0'}
    // Used Gate Set: ['rz', 'sx', 'x', 'cx', 'measure']

    OPENQASM 2.0;
    include \"qelib1.inc\";
    qreg q[3];
    qreg flag[1];
    creg meas[4];
    rz(pi/2) q[0];
    sx q[0];
    rz(pi/2) q[0];
    rz(pi/2) q[1];
    sx q[1];
    rz(pi/2) q[1];
    rz(pi/2) q[2];
    sx q[2];
    rz(pi/8) q[2];
    x flag[0];
    cx q[2],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],flag[0];
    rz(pi/8) flag[0];
    cx q[2],q[1];
    rz(-pi/8) q[1];
    cx q[1],flag[0];
    rz(pi/8) flag[0];
    cx q[1],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],q[1];
    rz(-3*pi/8) q[1];
    cx q[1],flag[0];
    rz(-pi/8) flag[0];
    cx q[1],flag[0];
    rz(pi/8) flag[0];
    cx q[1],q[0];
    rz(-pi/8) q[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],q[0];
    rz(pi/8) q[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[1],q[0];
    rz(-pi/8) q[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    sx q[1];
    rz(3*pi/4) q[1];
    cx q[2],q[0];
    rz(-3*pi/8) q[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    sx q[0];
    rz(5*pi/4) q[0];
    sx q[2];
    rz(pi/2) q[2];
    cx q[1],q[2];
    rz(-pi/4) q[2];
    cx q[0],q[2];
    rz(pi/4) q[2];
    cx q[1],q[2];
    rz(-pi/4) q[2];
    cx q[0],q[2];
    cx q[0],q[1];
    rz(-pi/4) q[1];
    cx q[0],q[1];
    sx q[0];
    rz(-pi/2) q[0];
    rz(pi/2) q[1];
    sx q[1];
    rz(-pi/2) q[1];
    rz(3*pi/4) q[2];
    sx q[2];
    rz(-7*pi/8) q[2];
    cx q[2],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],flag[0];
    rz(pi/8) flag[0];
    cx q[2],q[1];
    rz(-pi/8) q[1];
    cx q[1],flag[0];
    rz(pi/8) flag[0];
    cx q[1],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],q[1];
    rz(-3*pi/8) q[1];
    cx q[1],flag[0];
    rz(-pi/8) flag[0];
    cx q[1],flag[0];
    rz(pi/8) flag[0];
    cx q[1],q[0];
    rz(-pi/8) q[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[2],q[0];
    rz(pi/8) q[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[1],q[0];
    rz(-pi/8) q[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    sx q[1];
    rz(3*pi/4) q[1];
    cx q[2],q[0];
    rz(-3*pi/8) q[0];
    cx q[0],flag[0];
    rz(-pi/8) flag[0];
    cx q[0],flag[0];
    rz(pi/8) flag[0];
    sx q[0];
    rz(5*pi/4) q[0];
    sx q[2];
    rz(pi/2) q[2];
    cx q[1],q[2];
    rz(-pi/4) q[2];
    cx q[0],q[2];
    rz(pi/4) q[2];
    cx q[1],q[2];
    rz(-pi/4) q[2];
    cx q[0],q[2];
    cx q[0],q[1];
    rz(-pi/4) q[1];
    cx q[0],q[1];
    sx q[0];
    rz(-pi/2) q[0];
    rz(pi/2) q[1];
    sx q[1];
    rz(-pi/2) q[1];
    rz(3*pi/4) q[2];
    sx q[2];
    rz(-pi/2) q[2];
    barrier q[0],q[1],q[2],flag[0];
    measure q[0] -> meas[0];
    measure q[1] -> meas[1];
    measure q[2] -> meas[2];
    measure flag[0] -> meas[3];
";
