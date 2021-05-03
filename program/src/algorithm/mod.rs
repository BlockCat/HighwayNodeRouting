use crate::network::{EdgeId, Network, NodeId};

pub mod manymanydijkstra;
pub mod simple_a_star;

pub trait PathAlgorithm {
    type Network: Network;
    type Output;

    fn path(&self, source: NodeId, target: NodeId) -> Result<(Self::Output, Vec<EdgeId>), ()>;
}

pub trait ManyManyPathAlgorithm {
    type Network: Network;

    fn path(&self, nodes: &[NodeId]) -> Result<Vec<EdgePath>, ManyManyErrors>;
}

#[derive(Debug)]
pub struct EdgePath {
    pub source: NodeId,
    pub target: NodeId,
    pub edges: Vec<EdgeId>,
}

#[derive(Debug)]
pub enum ManyManyErrors {
    EmptyNodeList,
    NotAllPairsFound(Vec<EdgePath>),
}
