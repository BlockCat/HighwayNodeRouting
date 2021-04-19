pub mod aos_network;
pub mod consts;
pub mod network_lite;
pub mod utils;

use std::{
    error::Error,
    fs::File,
    io::{BufReader, Write},
    ops::{Index, IndexMut},
    path::Path,
};

pub use aos_network::{AoSNetwork, BuildEdge, BuildNode, Edge, Node};
pub use network_lite::LiteNetwork;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use shapefile::{reader::ShapeRecordIterator, Polyline};

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
        let reader = File::open(path)?;
        let network: T = bincode::deserialize_from(reader)?;
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
