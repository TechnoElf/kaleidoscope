use crate::circ::sequence::Gate::*;

#[derive(Debug, Clone)]
pub struct Circuit {
    q_lines: usize,
    c_lines: usize,
    gates: Vec<Gate>
}

#[allow(dead_code)]
impl Circuit {
    pub fn new() -> Self {
        Self {
            q_lines: 0,
            c_lines: 0,
            gates: Vec::new()
        }
    }

    pub fn lines(&self) -> (usize, usize) {
        (self.q_lines, self.c_lines)
    }

    pub fn push(&mut self, g: Gate) {
        match g {
            Measure { ql, cl } => self.measure(ql, cl),
            H { l } => self.h(l),
            X { l } => self.x(l),
            Y { l } => self.y(l),
            Z { l } => self.z(l),
            SX { l } => self.sx(l),
            RX { l, theta } => self.rx(l, &theta),
            RY { l, theta } => self.ry(l, &theta),
            RZ { l, theta } => self.rz(l, &theta),
            P { l, theta } => self.p(l, &theta),
            U2 { l, phi, lambda } => self.u2(l, &phi, &lambda),
            CX { l, c } => self.cx(l, c),
            CP { l, c, theta } => self.cp(l, c, &theta),
            CCX { l, c0, c1 } => self.ccx(l, c0, c1)
        }
    }

    pub fn gates(&self) -> &Vec<Gate> {
        &self.gates
    }
}

#[allow(dead_code)]
impl Circuit {
    pub fn measure(&mut self, ql: usize, cl: usize) {
        if self.q_lines <= ql { self.q_lines = ql + 1; }
        if self.c_lines <= cl { self.c_lines = cl + 1; }
        self.gates.push(Measure { ql, cl })
    }

    pub fn h(&mut self, l: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(H { l })
    }

    pub fn x(&mut self, l: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(X { l })
    }

    pub fn y(&mut self, l: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(Y { l })
    }

    pub fn z(&mut self, l: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(Z { l })
    }

    pub fn sx(&mut self, l: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(SX { l })
    }

    pub fn rx(&mut self, l: usize, theta: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(RX { l, theta: theta.to_string() })
    }

    pub fn ry(&mut self, l: usize, theta: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(RY { l, theta: theta.to_string() })
    }

    pub fn rz(&mut self, l: usize, theta: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(RZ { l, theta: theta.to_string() })
    }

    pub fn p(&mut self, l: usize, theta: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(P { l, theta: theta.to_string() })
    }

    pub fn u2(&mut self, l: usize, phi: &str, lambda: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        self.gates.push(U2 { l, phi: phi.to_string(), lambda: lambda.to_string() })
    }

    pub fn cx(&mut self, l: usize, c: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        if self.q_lines <= c { self.q_lines = c + 1; }
        self.gates.push(CX { l, c })
    }

    pub fn cp(&mut self, l: usize, c: usize, theta: &str) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        if self.q_lines <= c { self.q_lines = c + 1; }
        self.gates.push(CP { l, c, theta: theta.to_string() })
    }

    pub fn ccx(&mut self, l: usize, c0: usize, c1: usize) {
        if self.q_lines <= l { self.q_lines = l + 1; }
        if self.q_lines <= c0 { self.q_lines = c0 + 1; }
        if self.q_lines <= c1 { self.q_lines = c1 + 1; }
        self.gates.push(CCX { l, c0, c1 })
    }
}

#[derive(Debug, Clone, Eq)]
pub enum Gate {
    Measure { ql: usize, cl: usize },
    H { l: usize }, // Hadamard
    X { l: usize }, // X
    Y { l: usize }, // Y
    Z { l: usize }, // Z
    SX { l: usize }, // Square root of x
    RX { l: usize, theta: String }, // Rotate x
    RY { l: usize, theta: String }, // Rotate y
    RZ { l: usize, theta: String }, // Rotate z
    P { l: usize, theta: String }, // Phase
    U2 { l: usize, phi: String, lambda: String }, // Rotate y and z
    CX { l: usize, c: usize }, // Controlled x
    CP { l: usize, c: usize, theta: String }, // Controlled phase
    CCX { l: usize, c0: usize, c1: usize } // Toffoli
}

impl PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Measure { ql: qla, cl: cla }, Measure { ql: qlb, cl: clb }) => qla == qlb && cla == clb,
            (H { l: la }, H { l: lb }) => la == lb,
            (X { l: la }, X { l: lb }) => la == lb,
            (Y { l: la }, Y { l: lb }) => la == lb,
            (Z { l: la }, Z { l: lb }) => la == lb,
            (SX { l: la }, SX { l: lb }) => la == lb,
            (RX { l: la, theta: theta_a }, RX { l: lb, theta: theta_b }) => la == lb && theta_a == theta_b,
            (RY { l: la, theta: theta_a }, RY { l: lb, theta: theta_b }) => la == lb && theta_a == theta_b,
            (RZ { l: la, theta: theta_a }, RZ { l: lb, theta: theta_b }) => la == lb && theta_a == theta_b,
            (P { l: la, theta: theta_a }, P { l: lb, theta: theta_b }) => la == lb && theta_a == theta_b,
            (U2 { l: la, phi: phi_a, lambda: lambda_a }, U2 { l: lb, phi: phi_b, lambda: lambda_b }) => la == lb && phi_a == phi_b && lambda_a == lambda_b,
            (CX { l: la, c: ca }, CX { l: lb, c: cb }) => la == lb && ca == cb,
            (CP { l: la, c: ca, theta: theta_a }, CP { l: lb, c: cb, theta: theta_b }) => la == lb && ca == cb && theta_a == theta_b,
            (CCX { l: la, c0: c0a, c1: c1a }, CCX { l: lb, c0: c0b, c1: c1b }) => la == lb && c0a == c0b && c1a == c1b,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_circuit() {
        let mut circ = Circuit::new();

        circ.h(0);
        assert_eq!(circ.lines(), (1, 0));
    
        circ.push(Gate::H { l: 2 });
        assert_eq!(circ.lines(), (3, 0));

        circ.measure(0, 0);
        assert_eq!(circ.lines(), (3, 1));

        circ.x(0);
        circ.y(1);
        circ.z(2);
        circ.cx(0, 3);
        assert_eq!(circ.lines(), (4, 1));
    }
}
