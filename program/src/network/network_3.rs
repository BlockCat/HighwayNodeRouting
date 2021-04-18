use super::{EdgeId, Network, NodeCoord, NodeId};

// Metadata is added.
// Information in the network:
// Nodes:
// - id
// - junction_id
// - outgoing_edges
// - incoming_edges
// - edges
// - coords

// Edges:
// - source
// - target
// - distance

// Edge Metadata
// - streetname
// -

pub struct LiteNetwork {
    nodes: NodeData,
    edges: EdgeData,
}

impl Network for LiteNetwork {
    fn junction_id(&self, id: NodeId) -> usize {
        todo!()
    }

    fn outgoing_edges(&self, id: NodeId) -> &Vec<EdgeId> {
        todo!()
    }

    fn incoming_edges(&self, id: NodeId) -> &Vec<EdgeId> {
        todo!()
    }

    fn edge_source(&self, id: EdgeId) -> NodeId {
        todo!()
    }

    fn edge_target(&self, id: EdgeId) -> NodeId {
        todo!()
    }

    fn edge_object_id(&self, id: EdgeId) -> usize {
        todo!()
    }

    fn edge_distance(&self, id: EdgeId) -> f32 {
        todo!()
    }

    fn node_location(&self, id: NodeId) -> NodeCoord {
        todo!()
    }
}

struct NodeData {
    junctions: Vec<usize>,
    coordinate: Vec<NodeCoord>,
    outgoing_edges: Vec<Vec<EdgeId>>,
    incoming_edges: Vec<Vec<EdgeId>>,
}

struct EdgeData {
    object_id: Vec<usize>,
    source: Vec<NodeId>,
    target: Vec<NodeId>,
    distance: Vec<f32>,
}
