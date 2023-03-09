use crate::circ::sequence::Gate::*;

pub struct Circuit {
    q_lines: usize,
    c_lines: usize,
    gates: Vec<Gate>
}

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
            Z { l } => self.z(l)
        }
    }

    pub fn gates(&self) -> &Vec<Gate> {
        &self.gates
    }
}

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
}

#[derive(Debug, Clone, Copy, Eq)]
pub enum Gate {
    Measure { ql: usize, cl: usize },
    H { l: usize },
    X { l: usize },
    Y { l: usize },
    Z { l: usize }
}

impl PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Measure { .. }, Measure { .. }) => true,
            (H { .. }, H { .. }) => true,
            (X { .. }, X { .. }) => true,
            (Y { .. }, Y { .. }) => true,
            (Z { .. }, Z { .. }) => true,
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

        circ.x(0);
        circ.y(1);
        circ.z(2);
        assert_eq!(circ.lines(), (3, 0));

        circ.measure(0, 0);
        assert_eq!(circ.lines(), (3, 1));
    }
}
