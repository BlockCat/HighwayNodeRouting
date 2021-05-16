pub mod aos_network;
pub mod consts;
pub mod network_lite;
pub mod utils;

use crate::algorithm::dijkstra::DijkstraIterator;
pub use network_lite::LiteNetwork;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shapefile::{reader::ShapeRecordIterator, Polyline};
use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read, Write},
    ops::{Index, IndexMut},
    path::Path,
};

pub trait Network: From<ShapeRecordIterator<BufReader<File>, Polyline>> {
    fn nodes_len(&self) -> usize;
    fn junction_id(&self, id: NodeId) -> usize;
    fn outgoing_edges(&self, id: NodeId) -> &Vec<EdgeId>;
    fn incoming_edges(&self, id: NodeId) -> &Vec<EdgeId>;
    fn node_location(&self, id: NodeId) -> NodeCoord;

    fn edge_source(&self, id: EdgeId) -> NodeId;
    fn edge_target(&self, id: EdgeId) -> NodeId;
    fn edge_object_id(&self, id: EdgeId) -> usize;
    fn edge_distance(&self, id: EdgeId) -> f32;

    fn forward_dijkstra(&self, start: NodeId) -> DijkstraIterator<Self> {
        DijkstraIterator::new(
            self,
            start,
            crate::algorithm::dijkstra::DijkstraDirection::Forward,
        )
    }

    fn forward_radius_neighbourhood(&self, start: NodeId, radius: usize) -> Vec<(usize, NodeId)> {
        self.forward_dijkstra(start)
            .take_while(|(cost, _)| cost <= &radius)
            .collect()
    }

    fn backward_dijkstra(&self, start: NodeId) -> DijkstraIterator<Self> {
        DijkstraIterator::new(
            self,
            start,
            crate::algorithm::dijkstra::DijkstraDirection::Backward,
        )
    }

    fn backward_radius_neighbourhood(&self, start: NodeId, radius: usize) -> Vec<(usize, NodeId)> {
        self.backward_dijkstra(start)
            .take_while(|(cost, _)| cost <= &radius)
            .collect()
    }
}

pub trait Writeable: Sized {
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>>;
    fn read<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>>;
}

impl<T> Writeable for T
where
    T: Serialize + DeserializeOwned,
{
    fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let path: &Path = path.as_ref();
        let encoded = bincode::serialize(self)?;
        File::create(path)?.write_all(&encoded)?;
        Ok(())
    }

    fn read<P: AsRef<Path>>(path: P) -> Result<T, Box<dyn Error>> {
        let path: &Path = path.as_ref();
        let mut buffer = Vec::new();
        File::open(path)?.read_to_end(&mut buffer)?;
        let network: T = bincode::deserialize(&buffer)?;
        Ok(network)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub usize);
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

impl<T> IndexMut<NodeId> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        IndexMut::index_mut(self, index.0)
    }
}

impl<T> Index<EdgeId> for Vec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: EdgeId) -> &Self::Output {
        Index::index(self, index.0)
    }
}

impl<T> IndexMut<EdgeId> for Vec<T> {
    #[inline]
    fn index_mut(&mut self, index: EdgeId) -> &mut Self::Output {
        IndexMut::index_mut(self, index.0)
    }
}
