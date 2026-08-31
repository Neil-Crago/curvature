/// Graph kernel implementation for resonance nodes and edges.
/// Defines structures and methods for managing resonance graphs.
/// This module is part of the curvature crate.
/// A node in the resonance graph representing an observed semantic or field state.
#[allow(dead_code)]
pub struct ResonanceNode {
    /// Stable node identifier.
    id: usize,
    /// Local coherence value attached to the node.
    coherence: f64,
    /// Phase associated with the node.
    phase: f64,
}

/// An edge connecting two resonance nodes in the graph.
#[allow(dead_code)]
pub struct ResonanceEdge {
    /// Source node id.
    from: usize,
    /// Destination node id.
    to: usize,
    /// Edge amplitude or coupling strength.
    amplitude: f64,
    /// Oscillation frequency carried by the edge.
    frequency: f64,
}

/// A lightweight graph structure for resonance-network analysis.
pub struct GraphKernel {
    /// Nodes contained within the kernel.
    nodes: Vec<ResonanceNode>,
    /// Directed edges between nodes.
    edges: Vec<ResonanceEdge>,
}

impl ResonanceNode {
    /// Create a new resonance node with an identifier, coherence value, and phase.
    pub fn new(id: usize, coherence: f64, phase: f64) -> Self {
        ResonanceNode {
            id,
            coherence,
            phase,
        }
    }
}

impl ResonanceEdge {
    /// Create a new resonance edge connecting two node ids.
    pub fn new(from: usize, to: usize, amplitude: f64, frequency: f64) -> Self {
        ResonanceEdge {
            from,
            to,
            amplitude,
            frequency,
        }
    }
}

impl GraphKernel {
    /// Insert a node into the graph.
    pub fn add_node(&mut self, node: ResonanceNode) {
        self.nodes.push(node);
    }

    /// Insert an edge into the graph.
    pub fn add_edge(&mut self, edge: ResonanceEdge) {
        self.edges.push(edge);
    }

    /// Fetch a node by its identifier.
    pub fn get_node(&self, id: usize) -> Option<&ResonanceNode> {
        self.nodes.iter().find(|node| node.id == id)
    }

    /// Fetch a directed edge between two nodes, if present.
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&ResonanceEdge> {
        self.edges
            .iter()
            .find(|edge| edge.from == from && edge.to == to)
    }
}
