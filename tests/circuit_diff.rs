use kaleidoscope::circ::sequence::*;
use kaleidoscope::algo::diff::*;
use kaleidoscope::format::openqasm;

#[test]
fn simple_circuit_diff() {
    let mut a = Circuit::new();
    a.h(0);
    a.x(0);
    a.y(1);
    a.z(2);
    a.measure(2, 0);

    let mut b = Circuit::new();
    b.x(0);
    b.y(1);
    b.z(2);
    b.h(0);
    b.measure(0, 0);

    let edit_graph = EditGraph::new(a.gates().clone(), b.gates().clone());
    let edit_script = edit_graph.edit_script();

    let expected_script = vec![
        Edit::Remove(Gate::H { l: 0 }), Edit::Keep(Gate::X { l: 0 }),
        Edit::Keep(Gate::Y { l: 1 }), Edit::Keep(Gate::Z { l: 2 }),
        Edit::Insert(Gate::H { l: 0 }), Edit::Keep(Gate::Measure { ql: 2, cl: 0 })
    ];

    assert_eq!(edit_script, expected_script);
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

#[test]
fn openqasm_circuit_diff() {
    let a = openqasm::parse(SRCA.to_string());
    let b = openqasm::parse(SRCB.to_string());

    let edit_graph = EditGraph::new(a.gates().clone(), b.gates().clone());
    let _edit_script = edit_graph.edit_script();
}
