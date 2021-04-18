use crate::network::{EdgeId, Network, NodeId};

mod simple_a_star;

pub trait Algorithm {
    type Network: Network;
    type Output;

    fn path(&self, source: NodeId, target: NodeId) -> (Self::Output, Vec<EdgeId>);
}
