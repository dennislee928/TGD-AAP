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

    /// Build a deterministic feature-encoded circuit.
    ///
    /// Features are normalized into `[0, 1]`, then mapped to `RY` angles.
    /// A linear chain of CNOT gates entangles adjacent qubits to make
    /// correlations observable by the simulator.
    pub fn from_features(features: &[f32]) -> Self {
        let num_qubits = features.len().clamp(1, 8);
        let mut circuit = Self::new(num_qubits);

        for (idx, value) in features.iter().take(num_qubits).enumerate() {
            let normalized = ((*value as f64) + 1.0) / 2.0;
            let clamped = normalized.clamp(0.0, 1.0);
            let theta = clamped * std::f64::consts::PI;
            circuit.add_gate("ry", vec![idx], vec![theta]);
        }

        if num_qubits > 1 {
            for idx in 0..(num_qubits - 1) {
                circuit.add_gate("cx", vec![idx, idx + 1], vec![]);
            }
        }

        circuit
    }
}
