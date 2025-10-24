#[allow(dead_code)]
pub struct ResonanceNode {
    id: usize,
    coherence: f64,
    phase: f64,
}

#[allow(dead_code)]
pub struct ResonanceEdge {
    from: usize,
    to: usize,
    amplitude: f64,
    frequency: f64,
}

pub struct GraphKernel {
    nodes: Vec<ResonanceNode>,
    edges: Vec<ResonanceEdge>,
}


impl ResonanceNode {
    pub fn new(id: usize, coherence: f64, phase: f64) -> Self {
        ResonanceNode { id, coherence, phase }
    }
}

impl ResonanceEdge {
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
    pub fn add_node(&mut self, node: ResonanceNode) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, edge: ResonanceEdge) {
        self.edges.push(edge);
    }

    pub fn get_node(&self, id: usize) -> Option<&ResonanceNode> {
        self.nodes.iter().find(|node| node.id == id)
    }
    pub fn get_edge(&self, from: usize, to: usize) -> Option<&ResonanceEdge> {
        self.edges
            .iter()
            .find(|edge| edge.from == from && edge.to == to)
    }
}
