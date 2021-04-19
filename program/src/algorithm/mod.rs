use crate::network::{EdgeId, Network, NodeId};

pub mod simple_a_star;

pub trait Algorithm {
    type Network: Network;
    type Output;

    fn path(&self, source: NodeId, target: NodeId) -> Result<(Self::Output, Vec<EdgeId>), ()>;
}
