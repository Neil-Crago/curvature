/// Entangle map implementation for semantic domains.
use crate::resonance::EntangleMap;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticDomain {
    Biological,
    Quantum,
    Linguistic,
    Cognitive,
}

#[derive(Clone)]
pub struct Coupling {
    _strength: f64,
    _phase_shift: f64,
}

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
