use crate::network::{EdgeId, Network, NodeId};

pub mod dijkstra_bi_dir;
pub mod dijkstra;
pub mod simple_a_star;

pub trait PathAlgorithm {
    type Network: Network;
    type Output;

    fn path(&self, source: NodeId, target: NodeId) -> Result<(Self::Output, Vec<EdgeId>), ()>;
}

pub trait ManyToManyAlgorithm {
    type Network: Network;

    fn new(network: Self::Network) -> Self;
    fn network(&self) -> &Self::Network;
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
