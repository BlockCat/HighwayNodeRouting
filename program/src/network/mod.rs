pub mod aos_network;
mod network_3;

use std::ops::Index;

pub use aos_network::{AoSNetwork, BuildEdge, BuildNode, Edge, Node, RoadDirection};

pub trait Network {
    fn junction_id(&self, id: NodeId) -> usize;
    fn outgoing_edges(&self, id: NodeId) -> &Vec<EdgeId>;
    fn incoming_edges(&self, id: NodeId) -> &Vec<EdgeId>;
    fn node_location(&self, id: NodeId) -> NodeCoord;

    fn edge_source(&self, id: EdgeId) -> NodeId;
    fn edge_target(&self, id: EdgeId) -> NodeId;
    fn edge_object_id(&self, id: EdgeId) -> usize;
    fn edge_distance(&self, id: EdgeId) -> f32;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EdgeId(pub usize);
#[derive(Debug, Clone, Copy)]
pub struct NodeCoord {
    pub x: f32,
    pub y: f32,
}

impl NodeCoord {
    pub fn distance_squared(&self, other: &Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance_squared(other).sqrt()
    }
}

impl<T> Index<NodeId> for Vec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: NodeId) -> &Self::Output {
        Index::index(self, index.0)
    }
}

impl<T> Index<EdgeId> for Vec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: EdgeId) -> &Self::Output {
        Index::index(self, index.0)
    }
}
