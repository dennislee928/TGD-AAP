// holyQASM Quantum Logic Circuit — TGD-AAP
// Purpose: Quantum Feature Map for enhancing model input representations.
// Reference: Quantum Kernel Estimation via angle encoding.

// Allocate qubit register
QREG q[4];
// Classical register for measurement output
CREG c[4];

// --- Layer 1: Hadamard encoding (superposition) ---
H q[0];
H q[1];
H q[2];
H q[3];

// --- Layer 2: Angle encoding of input features ---
// Feature values are substituted at runtime by the quantum_bridge runner.
RZ(feature_0) q[0];
RZ(feature_1) q[1];
RZ(feature_2) q[2];
RZ(feature_3) q[3];

// --- Layer 3: Entanglement layer (CNOT ladder) ---
CNOT q[0], q[1];
CNOT q[1], q[2];
CNOT q[2], q[3];
CNOT q[3], q[0];

// --- Layer 4: Second Hadamard layer ---
H q[0];
H q[1];
H q[2];
H q[3];

// --- Measurement ---
MEASURE q[0] -> c[0];
MEASURE q[1] -> c[1];
MEASURE q[2] -> c[2];
MEASURE q[3] -> c[3];
