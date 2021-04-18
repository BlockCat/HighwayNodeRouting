use super::{EdgeId, Network, NodeId};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs::File, io::Write, path::Path, str::FromStr};
#[derive(Debug, Serialize, Deserialize)]
pub struct AoSNetwork {
    pub node_map: HashMap<usize, usize>,
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: usize,
    junction_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    id: usize,
    related: Option<usize>,
    pub distance: f32,
    target: usize,
}

#[derive(Debug)]
pub struct BuildNode {
    pub junction_id: usize,
}
#[derive(Debug)]
pub struct BuildEdge {
    pub source_node: usize,
    pub target_node: usize,
    pub distance: f32,
    pub direction: RoadDirection,
}

impl Network for AoSNetwork {
    fn junction_id(&self, id: NodeId) -> usize {
        self.nodes[id].junction_id
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

    fn node_location(&self, id: NodeId) -> super::NodeCoord {
        todo!()
    }
}

impl AoSNetwork {
    pub fn new() -> Self {
        Self {
            node_map: HashMap::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: BuildNode) -> usize {
        if let Some(id) = self.node_map.get(&node.junction_id) {
            *id
        } else {
            let id = self.nodes.len();

            self.nodes.push(Node {
                id,
                junction_id: node.junction_id,
            });
            self.edges.push(Vec::new());
            self.node_map.insert(node.junction_id, id);

            id
        }
    }

    pub fn add_edge(&mut self, edge: BuildEdge) {
        let source_node = &edge.source_node;
        let target_node = &edge.target_node;

        match edge.direction {
            RoadDirection::BOTH => {
                let source_edge_id = self.edges[*source_node].len();
                let target_edge_id = self.edges[*target_node].len();
                self.edges[*source_node].push(Edge {
                    id: source_edge_id,
                    related: Some(target_edge_id),
                    distance: edge.distance,
                    target: *target_node,
                });
                self.edges[*target_node].push(Edge {
                    id: target_edge_id,
                    distance: edge.distance,
                    related: Some(source_edge_id),
                    target: *source_node,
                });
            }
            RoadDirection::WITH => {
                let source_edge_id = self.edges[*source_node].len();
                self.edges[*source_node].push(Edge {
                    id: source_edge_id,
                    related: None,
                    distance: edge.distance,
                    target: *target_node,
                });
            }
            RoadDirection::AGAINST => {
                let target_edge_id = self.edges[*target_node].len();
                self.edges[*target_node].push(Edge {
                    id: target_edge_id,
                    distance: edge.distance,
                    related: None,
                    target: *source_node,
                });
            }
        }
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let path: &Path = path.as_ref();
        let encoded = bincode::serialize(self)?;
        File::create(path)?.write_all(&encoded)?;
        Ok(())
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<AoSNetwork, Box<dyn Error>> {
        let path: &Path = path.as_ref();
        let reader = File::open(path)?;
        let network: AoSNetwork = bincode::deserialize_from(reader)?;
        Ok(network)
    }
}

#[derive(Debug)]
pub enum RoadDirection {
    // Both (JTE_BEGIN <-> JTE_END) denoted with H
    BOTH,
    // Direction with flow: (JTE_BEGIN -> JTE_END) denoted with H
    WITH,
    // Direction against flow: (JTE_END -> JTE_BEGIN) denoted with T
    AGAINST,
}

impl FromStr for RoadDirection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "H" => Ok(RoadDirection::WITH),
            "T" => Ok(RoadDirection::AGAINST),
            "B" => Ok(RoadDirection::BOTH),
            "O" => Ok(RoadDirection::BOTH), // O (Onbekend) means unknown but used as both directions.
            _ => Err(s.into()),
        }
    }
}
