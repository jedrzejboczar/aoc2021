pub struct Graph<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Edge>,
}

pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct Node<T> {
    first_edge: Option<EdgeIndex>,  // a linked list of edges
    data: T,
}

pub struct Edge {
    target: NodeIndex,  // node at which this edge points
    next_edge: Option<EdgeIndex>,
}

// Needed:
// * add nodes
// * add connections
// * traverse the graph

impl<T> Graph<T> {
    pub fn get_node(&self, i: NodeIndex) -> &Node<T> {
        &self.nodes[i]
    }
}

impl<T> Node<T> {
    pub fn edges(&self) -> Vec<EdgeIndex> {
        let edges = Vec::new();
        if let Some(edge) = self.first_edge {
            todo!()
        }
        edges
    }
}
