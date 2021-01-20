use gdnative::api::*;
use gdnative::prelude::*;

use quantum_sim::{Gate, QuantumCircuit};

/// The QuantumSim "class"
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct QuantumSim {
    quantum_circuit: Option<QuantumCircuit>,
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl QuantumSim {
    // Register the builder for methods, properties and/or signals.
    fn register_builder(_builder: &ClassBuilder<Self>) {
        godot_print!("QuantumSim builder is registered!");
    }

    /// The "constructor" of the class.
    fn new(_owner: &Node) -> Self {
        godot_print!("QuantumSim is created!");
        Self {
            quantum_circuit: None,
        }
    }

    #[export]
    unsafe fn create_circuit(
        &mut self,
        _owner: &Node,
        num_qubits: usize,
        initial_states: Vec<usize>,
    ) {
        self.quantum_circuit = Some(QuantumCircuit::from_states(num_qubits, initial_states));
        if let Some(circuit) = self.quantum_circuit.as_ref() {
            godot_print!("{}", circuit.state());
        }
    }

    #[export]
    unsafe fn apply_hadamard(&mut self, _owner: &Node, qubit: usize) {
        if let Some(circuit) = self.quantum_circuit.as_mut() {
            circuit.apply_gate(Gate::H(qubit));
            godot_print!("{}", circuit.state());
        }
    }

    #[export]
    unsafe fn apply_not(&mut self, _owner: &Node, qubit: usize) {
        if let Some(circuit) = self.quantum_circuit.as_mut() {
            circuit.apply_gate(Gate::X(qubit));
            godot_print!("{}", circuit.state());
        }
    }

    #[export]
    unsafe fn apply_cnot(&mut self, _owner: &Node, control: usize, target: usize) {
        if let Some(circuit) = self.quantum_circuit.as_mut() {
            circuit.apply_gate(Gate::CX(control, target));
            godot_print!("{}", circuit.state());
        }
    }

    #[export]
    unsafe fn state(&mut self, _owner: &Node) -> Vec<f64> {
        if let Some(circuit) = self.quantum_circuit.as_mut() {
            let state = circuit.state();
            let mut vector = Vec::new();
            for i in state.iter() {
                vector.push(i.norm_sqr());
            }
            vector
        } else {
            Vec::new()
        }
    }

    #[export]
    unsafe fn measure(&mut self, _owner: &Node) -> usize {
        if let Some(circuit) = self.quantum_circuit.as_mut() {
            let result = circuit.measure();
            godot_print!("{}", circuit.state());
            result
        } else {
            0
        }
    }
}
