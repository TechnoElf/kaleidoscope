use crate::circ::sequence::Circuit;
use crate::format::openqasm::Instruction::*;

use std::collections::HashMap;

// https://arxiv.org/pdf/1707.03429.pdf

// TODO: use &str
// TODO: better error handling
pub fn parse(src: String) -> Circuit {
    let normal = normalise(src);
    let tokens = tokenise(normal);
    let syntax = analyse_syntax(tokens);
    let semantics = analyse_semantics(syntax);

    semantics
}

// -- preprocessing --

fn normalise(src: String) -> String {
    let src = remove_comments(src);
    let src = remove_whitespace(src);
    src
}

fn remove_comments(src: String) -> String {
    src.lines()
        .map(|l| l.split_once("//").map(|(s, _)| s).unwrap_or(l))
        .fold(String::new(), |mut acc, l| { acc.push_str(l); acc.push_str("\n"); acc })
}

fn remove_whitespace(src: String) -> String {
    src.lines().map(|l| l.trim()).collect()
}

// -- lexical analysis --

fn tokenise(src: String) -> Vec<Vec<String>> {
    let mut statements: Vec<String> = src.split(";")
        .map(|s| { let mut s = s.to_string(); s.push(';'); s })
        .collect();
    statements.pop();
    let tokens = statements.into_iter().map(|s| tokenise_statement(s)).collect();

    tokens
}

fn tokenise_statement(s: String) -> Vec<String> {
    let mut chars: Vec<char> = s.chars().collect();
 
    let mut tokens = Vec::new();
    let mut cur = String::new();

    while !chars.is_empty() {
        let c = chars.remove(0);

        if c.is_whitespace() {
            if !cur.is_empty() {
                tokens.push(cur);
                cur = String::new();
            }
            continue;
        }

        if c == '(' || c == ')' || c == '[' || c == ']' || c == ',' {
            if !cur.is_empty() {
                tokens.push(cur);
                cur = String::new();
            }
            tokens.push(c.to_string());
            continue;
        }

        if c == ';' {
            if !cur.is_empty() {
                tokens.push(cur);
            }
            break;
        }

        cur.push(c)
    }

    tokens
}

// -- syntax analysis --

#[derive(Debug, Clone)]
enum Instruction {
    Ver,
    Incl,
    QRegDef { id: String, size: usize },
    CRegDef { id: String, size: usize },
    Measure { idq: String, iq: usize, idc: String, ic: usize },
    QOp { id: String, args: Vec<(String, usize)> },
}

// TODO: syntax checks
fn analyse_syntax(tokens: Vec<Vec<String>>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for mut s in tokens.into_iter() {
        match s[0].as_str() {
            "OPENQASM" => instructions.push(Ver),
            "include" => instructions.push(Incl),
            "qreg" => instructions.push(QRegDef { id: s[1].clone(), size: s[3].parse().unwrap() }),
            "creg" => instructions.push(CRegDef { id: s[1].clone(), size: s[3].parse().unwrap() }),
            "measure" => instructions.push(Measure { idq: s[1].clone(), iq: s[3].parse().unwrap(), idc: s[6].clone(), ic: s[8].parse().unwrap() }),
            "barrier" => (),
            _ => {
                let instr = s.remove(0);

                if s[0] == "(" {
                    while s[0] != ")" {
                        s.remove(0);
                    }
                    let _ = s.remove(0);
                }

                let mut args = Vec::new();
                while !s.is_empty() {
                    let id = s.remove(0);
                    let _ = s.remove(0);
                    let i = s.remove(0).parse().unwrap();
                    let _ = s.remove(0);

                    args.push((id, i));

                    if !s.is_empty() {
                        let _ = s.remove(0);
                    }
                }

                instructions.push(QOp { id: instr, args });
            }
        }
    }

    instructions
}

// -- semantic analysis --

// TODO: semantics checks
fn analyse_semantics(syntax: Vec<Instruction>) -> Circuit {
    let symbol_table = build_symbol_table(&syntax);
    let mut circ = Circuit::new();

    for instr in syntax {
        match instr {
            Measure { idq, iq, idc, ic } => {
                circ.measure(
                    symbol_table[&idq].line_offset + iq,
                    symbol_table[&idc].line_offset + ic,
                );
            },
            QOp { id, args } => {
                match id.as_str() {
                    "h" => { circ.h(symbol_table[&args[0].0].line_offset + args[0].1); },
                    "x" => { circ.x(symbol_table[&args[0].0].line_offset + args[0].1); },
                    "y" => { circ.y(symbol_table[&args[0].0].line_offset + args[0].1); },
                    "z" => { circ.z(symbol_table[&args[0].0].line_offset + args[0].1); },
                    "cp" => (),
                    "cx" => {
                        circ.cx(
                            symbol_table[&args[1].0].line_offset + args[1].1,
                            symbol_table[&args[0].0].line_offset + args[0].1
                        );
                    },
                    "u2" => (),
                    "u1" => (),
                    "ccx" => {
                        circ.ccx(
                            symbol_table[&args[2].0].line_offset + args[2].1,
                            symbol_table[&args[1].0].line_offset + args[1].1,
                            symbol_table[&args[0].0].line_offset + args[0].1
                        );
                    },
                    "rz" => (),
                    "sx" => { circ.sx(symbol_table[&args[0].0].line_offset + args[0].1); },
                    _ => unimplemented!("unimplemented instruction \"{}\"", id)
                }
            }
            _ => ()
        }
    }

    circ
}

#[derive(Debug, Clone)]
struct SymbolData {
    line_offset: usize,
    size: usize,
    is_classical: bool
}

fn build_symbol_table(syntax: &Vec<Instruction>) -> HashMap<String, SymbolData> {
    let mut table = HashMap::new();
    
    let mut q_lines = 0;
    let mut c_lines = 0;

    for instr in syntax {
        match instr {
            QRegDef { id, size } => {
                table.insert(id.clone(), SymbolData {
                    line_offset: q_lines,
                    size: *size,
                    is_classical: false
                });
                q_lines += size;
            },
            CRegDef { id, size } => {
                table.insert(id.clone(), SymbolData {
                    line_offset: c_lines,
                    size: *size,
                    is_classical: true
                });
                c_lines += size;
            },
            _ => ()
        }
    }

    table
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC0: &str = "
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

    #[test]
    fn parse_source() {
        let _circ = parse(SRC0.to_string());
    }
}
