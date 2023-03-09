use kaleidoscope::circ::sequence::*;
use kaleidoscope::algo::diff::*;

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
