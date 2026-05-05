//! quantum_bridge/mod.rs — Rust interface to holyQASM quantum logic.
//!
//! Provides a safe Rust API for scheduling and retrieving results from
//! holyQASM quantum circuit simulations via FFI or subprocess calls.

pub mod circuit;
pub mod runner;

use circuit::QuantumCircuit;

/// Output of feature scoring through the quantum bridge.
#[derive(Debug, Clone)]
pub struct BridgeInference {
    pub prediction: f32,
    pub confidence: f32,
    pub used_quantum: bool,
}

/// Score a feature vector with a quantum-first path and deterministic fallback.
///
/// If the simulator is unavailable, malformed, or returns unusable amplitudes,
/// this function falls back to a deterministic classical score.
pub async fn score_features(features: &[f32]) -> BridgeInference {
    let fallback = classical_fallback(features);
    let circuit = QuantumCircuit::from_features(features);

    match runner::run_circuit(&circuit).await {
        Ok(sim) if !sim.amplitudes.is_empty() => {
            let total: f64 = sim.amplitudes.iter().copied().sum();
            if total <= f64::EPSILON {
                return fallback;
            }

            let dominant = sim
                .amplitudes
                .get(sim.max_state)
                .copied()
                .unwrap_or_default()
                .max(0.0);
            let ratio = (dominant / total).clamp(0.0, 1.0);
            let prediction = ratio as f32;
            let confidence = (0.55_f32 + 0.45_f32 * prediction).clamp(0.0, 1.0);

            BridgeInference {
                prediction,
                confidence,
                used_quantum: true,
            }
        }
        _ => fallback,
    }
}

fn classical_fallback(features: &[f32]) -> BridgeInference {
    if features.is_empty() {
        return BridgeInference {
            prediction: 0.0,
            confidence: 0.5,
            used_quantum: false,
        };
    }

    let mut weighted_sum = 0.0_f64;
    let mut denom = 0.0_f64;
    for (idx, value) in features.iter().enumerate() {
        let weight = (idx + 1) as f64;
        weighted_sum += (*value as f64) * weight;
        denom += weight;
    }

    let normalized = (weighted_sum / denom).clamp(-8.0, 8.0);
    let prediction = (1.0 / (1.0 + (-normalized).exp())) as f32;
    let confidence = (0.5_f32 + 0.4_f32 * (prediction - 0.5_f32).abs() * 2.0_f32).clamp(0.0, 1.0);

    BridgeInference {
        prediction,
        confidence,
        used_quantum: false,
    }
}
