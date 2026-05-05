//! quantum_bridge/runner.rs — holyQASM circuit execution interface.
//!
//! Executes a QuantumCircuit by invoking the holyQASM simulator as a
//! subprocess and parsing the resulting probability amplitudes.

use anyhow::{Context, Result};
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::{info, warn};

use super::circuit::QuantumCircuit;

static TEMP_FILE_SEQ: AtomicU64 = AtomicU64::new(0);

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

    // Write the QASM source to a unique temp file to avoid cross-process collisions.
    let tmp_path = std::env::temp_dir().join(format!(
        "circuit-{}-{}-{}.qasm",
        std::process::id(),
        TEMP_FILE_SEQ.fetch_add(1, Ordering::Relaxed),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));
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

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let _ = tokio::fs::remove_file(&tmp_path).await;
        anyhow::bail!(
            "holyQASM exited with status {}. stderr: {} stdout: {}",
            output
                .status
                .code()
                .map_or_else(|| "terminated by signal".to_string(), |c| c.to_string()),
            stderr.trim(),
            stdout.trim()
        );
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let amplitudes: Vec<f64> =
        serde_json::from_str(&stdout).context("Failed to parse holyQASM JSON output")?;

    if let Err(err) = tokio::fs::remove_file(&tmp_path).await {
        warn!(path = ?tmp_path, error = %err, "Failed to remove QASM temp file");
    }

    let max_state = amplitudes
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .map(|(i, _)| i)
        .unwrap_or(0);

    Ok(SimulationResult { amplitudes, max_state })
}
