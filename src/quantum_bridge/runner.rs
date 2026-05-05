//! quantum_bridge/runner.rs — holyQASM circuit execution interface.
//!
//! Executes a QuantumCircuit by invoking the holyQASM simulator as a
//! subprocess and parsing the resulting probability amplitudes.

use anyhow::{Context, Result};
use tracing::info;

use super::circuit::QuantumCircuit;

/// The result of executing a quantum circuit simulation.
#[derive(Debug)]
pub struct SimulationResult {
    /// Probability amplitude for each basis state.
    pub amplitudes: Vec<f64>,
    /// Index of the most probable measurement outcome.
    pub max_state: usize,
}

/// Run a circuit through the holyQASM simulator binary and return results.
pub async fn run_circuit(circuit: &QuantumCircuit) -> Result<SimulationResult> {
    let qasm_source = circuit.to_qasm();

    // Write the QASM source to a temporary file.
    let tmp_path = std::env::temp_dir().join("circuit.qasm");
    tokio::fs::write(&tmp_path, &qasm_source)
        .await
        .context("Failed to write QASM temp file")?;

    info!("Running holyQASM simulation for circuit with {} qubits", circuit.num_qubits);

    let output = tokio::process::Command::new("holyqasm")
        .arg("--run")
        .arg(&tmp_path)
        .arg("--output=json")
        .output()
        .await
        .context("Failed to invoke holyQASM binary — ensure it is installed")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let amplitudes: Vec<f64> = serde_json::from_str(&stdout)
        .unwrap_or_else(|_| vec![0.0; 1 << circuit.num_qubits]);

    let max_state = amplitudes
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);

    Ok(SimulationResult { amplitudes, max_state })
}
