// Benchmark was created by MQT Bench on 2022-12-14
// For more information about MQT Bench, please visit https://www.cda.cit.tum.de/mqtbench/
// MQT Bench version: 0.2.2
// Qiskit version: {'qiskit-terra': '0.22.3', 'qiskit-aer': '0.11.1', 'qiskit-ignis': None, 'qiskit-ibmq-provider': '0.19.2', 'qiskit': '0.39.3', 'qiskit-nature': '0.5.2', 'qiskit-finance': '0.3.4', 'qiskit-optimization': '0.4.0', 'qiskit-machine-learning': '0.5.0'}
// Used Gate Set: ['rz', 'sx', 'x', 'cx', 'measure']

OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
qreg flag[1];
creg meas[2];
rz(pi/2) q[0];
sx q[0];
rz(pi/2) q[0];
x flag[0];
barrier q[0],flag[0];
measure q[0] -> meas[0];
measure flag[0] -> meas[1];
