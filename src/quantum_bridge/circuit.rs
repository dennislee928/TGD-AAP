//! quantum_bridge/circuit.rs — Quantum circuit definition helpers.
//!
//! Provides typed structures that map to holyQASM gate instructions,
//! enabling Rust code to compose quantum circuits programmatically.

use serde::{Deserialize, Serialize};

/// A single quantum gate instruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateInstruction {
    pub gate: String,
    pub qubits: Vec<usize>,
    pub params: Vec<f64>,
}

/// A full quantum circuit composed of ordered gate instructions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub instructions: Vec<GateInstruction>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            instructions: Vec::new(),
        }
    }

    /// Append a gate instruction to the circuit.
    pub fn add_gate(&mut self, gate: impl Into<String>, qubits: Vec<usize>, params: Vec<f64>) {
        self.instructions.push(GateInstruction {
            gate: gate.into(),
            qubits,
            params,
        });
    }

    /// Serialize the circuit to a holyQASM-compatible string.
    pub fn to_qasm(&self) -> String {
        let mut out = format!("QREG q[{}];\n", self.num_qubits);
        for instr in &self.instructions {
            let params_str = if instr.params.is_empty() {
                String::new()
            } else {
                format!(
                    "({})",
                    instr
                        .params
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                )
            };
            let qubits_str = instr
                .qubits
                .iter()
                .map(|q| format!("q[{}]", q))
                .collect::<Vec<_>>()
                .join(",");
            out.push_str(&format!("{}{} {};\n", instr.gate, params_str, qubits_str));
        }
        out
    }
}
