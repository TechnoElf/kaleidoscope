// Benchmark was created by MQT Bench on 2022-12-14
// For more information about MQT Bench, please visit https://www.cda.cit.tum.de/mqtbench/
// MQT Bench version: 0.2.2
// Qiskit version: {'qiskit-terra': '0.22.3', 'qiskit-aer': '0.11.1', 'qiskit-ignis': None, 'qiskit-ibmq-provider': '0.19.2', 'qiskit': '0.39.3', 'qiskit-nature': '0.5.2', 'qiskit-finance': '0.3.4', 'qiskit-optimization': '0.4.0', 'qiskit-machine-learning': '0.5.0'}
// Used Gate Set: ['rz', 'sx', 'x', 'cx', 'measure']
// Coupling List: [[0, 1], [1, 0], [1, 2], [1, 4], [2, 1], [2, 3], [3, 2], [3, 5], [4, 1], [4, 7], [5, 3], [5, 8], [6, 7], [7, 4], [7, 6], [7, 10], [8, 5], [8, 9], [8, 11], [9, 8], [10, 7], [10, 12], [11, 8], [11, 14], [12, 10], [12, 13], [12, 15], [13, 12], [13, 14], [14, 11], [14, 13], [14, 16], [15, 12], [15, 18], [16, 14], [16, 19], [17, 18], [18, 15], [18, 17], [18, 21], [19, 16], [19, 20], [19, 22], [20, 19], [21, 18], [21, 23], [22, 19], [22, 25], [23, 21], [23, 24], [24, 23], [24, 25], [25, 22], [25, 24], [25, 26], [26, 25]]

OPENQASM 2.0;
include "qelib1.inc";
qreg q[27];
creg c[3];
rz(pi/2) q[22];
sx q[22];
rz(pi) q[22];
rz(pi/2) q[24];
sx q[24];
rz(pi) q[24];
rz(pi/2) q[25];
sx q[25];
rz(-pi/2) q[25];
cx q[24],q[25];
cx q[22],q[25];
sx q[22];
rz(pi/2) q[22];
sx q[24];
rz(pi/2) q[24];
rz(pi/2) q[26];
sx q[26];
rz(pi) q[26];
cx q[26],q[25];
sx q[26];
rz(pi/2) q[26];
barrier q[24],q[22],q[26],q[25];
measure q[24] -> c[0];
measure q[22] -> c[1];
measure q[26] -> c[2];
