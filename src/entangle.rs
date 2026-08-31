/// Entangle map implementation for semantic domains.
use crate::resonance::EntangleMap;
use std::collections::HashMap;

/// Identifies a semantic domain used by the entanglement model.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticDomain {
    /// Biological or living-system domain.
    Biological,
    /// Quantum or probabilistic domain.
    Quantum,
    /// Linguistic or symbolic domain.
    Linguistic,
    /// Cognitive or reasoning domain.
    Cognitive,
}

/// Describes the interaction between two semantic domains.
#[derive(Clone)]
pub struct Coupling {
    /// Effective interaction strength.
    _strength: f64,
    /// Phase offset between domains.
    _phase_shift: f64,
}

/// A simple in-memory map of domain-to-domain coupling values.
pub struct SimpleEntangleMap {
    map: HashMap<(SemanticDomain, SemanticDomain), Coupling>,
}

impl EntangleMap for SimpleEntangleMap {
    type Domain = SemanticDomain;
    type Coupling = Coupling;

    fn new() -> Self {
        SimpleEntangleMap {
            map: HashMap::new(),
        }
    }

    fn get_coupling(&self, a: &SemanticDomain, b: &SemanticDomain) -> Coupling {
        self.map
            .get(&(a.clone(), b.clone()))
            .cloned()
            .unwrap_or(Coupling {
                _strength: 0.0,
                _phase_shift: 0.0,
            })
    }

    fn update_coupling(&mut self, a: &SemanticDomain, b: &SemanticDomain, delta: Coupling) {
        self.map.insert((a.clone(), b.clone()), delta);
    }
}
