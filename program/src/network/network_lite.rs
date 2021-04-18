use super::{EdgeId, Network, NodeCoord, NodeId};
use crate::network::{consts::*, utils::*};
use serde::{Deserialize, Serialize};
use shapefile::{reader::ShapeRecordIterator, Polyline};
use std::{collections::HashMap, fs::File, io::BufReader};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LiteNetwork {
    nodes: NodeData,
    edges: EdgeData,
}

impl LiteNetwork {
    pub fn node_len(&self) -> usize {
        self.nodes.junctions.len()
    }
    pub fn edge_len(&self) -> usize {
        self.edges.object_id.len()
    }
}

impl Network for LiteNetwork {
    fn junction_id(&self, id: NodeId) -> usize {
        self.nodes.junctions[id]
    }

    fn outgoing_edges(&self, id: NodeId) -> &Vec<EdgeId> {
        &self.nodes.outgoing_edges[id]
    }

    fn incoming_edges(&self, id: NodeId) -> &Vec<EdgeId> {
        &self.nodes.incoming_edges[id]
    }

    fn node_location(&self, id: NodeId) -> NodeCoord {
        self.nodes.coordinate[id]
    }

    fn edge_source(&self, id: EdgeId) -> NodeId {
        self.edges.source[id]
    }

    fn edge_target(&self, id: EdgeId) -> NodeId {
        self.edges.target[id]
    }

    fn edge_object_id(&self, id: EdgeId) -> usize {
        self.edges.object_id[id]
    }

    fn edge_distance(&self, id: EdgeId) -> f32 {
        self.edges.distance[id]
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct NodeData {
    junctions: Vec<usize>,
    coordinate: Vec<NodeCoord>,
    outgoing_edges: Vec<Vec<EdgeId>>,
    incoming_edges: Vec<Vec<EdgeId>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct EdgeData {
    object_id: Vec<usize>,
    source: Vec<NodeId>,
    target: Vec<NodeId>,
    distance: Vec<f32>,
}

impl LiteNetwork {
    pub fn new() -> Self {
        Self {
            nodes: NodeData {
                coordinate: Vec::new(),
                incoming_edges: Vec::new(),
                outgoing_edges: Vec::new(),
                junctions: Vec::new(),
            },
            edges: EdgeData {
                object_id: Vec::new(),
                source: Vec::new(),
                target: Vec::new(),
                distance: Vec::new(),
            },
        }
    }

    fn add_node(&mut self, junction_id: usize, coordinate: NodeCoord) -> NodeId {
        self.nodes.outgoing_edges.push(Vec::new());
        self.nodes.incoming_edges.push(Vec::new());
        self.nodes.junctions.push(junction_id);
        self.nodes.coordinate.push(coordinate);
        NodeId(self.nodes.junctions.len() - 1)
    }

    fn add_edge(
        &mut self,
        object_id: usize,
        source: NodeId,
        target: NodeId,
        distance: f32,
    ) -> EdgeId {
        self.edges.object_id.push(object_id);
        self.edges.source.push(source);
        self.edges.target.push(target);
        self.edges.distance.push(distance);
        EdgeId(self.edges.object_id.len() - 1)
    }
}

impl From<ShapeRecordIterator<BufReader<File>, Polyline>> for LiteNetwork {
    fn from(shapes: ShapeRecordIterator<BufReader<File>, Polyline>) -> Self {
        let mut network = LiteNetwork::new();
        let mut mapping = HashMap::new();
        let mut shape_counter = 0;

        for index in shapes {
            let (shape, record) = index.unwrap();

            let direction: RoadDirection =
                get_character(&record, DIRECTION).unwrap().parse().unwrap();
            let junction_start = get_numeric(&record, NODE_START).unwrap() as usize;
            let junction_end = get_numeric(&record, NODE_END).unwrap() as usize;

            let node_start_id = if let Some(id) = mapping.get(&junction_start) {
                *id
            } else {
                assert!(shape.parts().len() == 1);
                let p = shape.part(0).and_then(|x| x.first()).unwrap();
                let id = network.add_node(
                    junction_start,
                    NodeCoord {
                        x: p.x as f32,
                        y: p.y as f32,
                    },
                );
                mapping.insert(junction_start, id);
                id
            };
            let node_end_id = if let Some(id) = mapping.get(&junction_end) {
                *id
            } else {
                assert!(shape.parts().len() == 1);
                let p = shape.part(0).and_then(|x| x.last()).unwrap();
                let id = network.add_node(
                    junction_end,
                    NodeCoord {
                        x: p.x as f32,
                        y: p.y as f32,
                    },
                );
                mapping.insert(junction_start, id);
                id
            };

            let node_start_id = node_start_id;
            let node_end_id = node_end_id;

            let shape_distance = calculate_distance(&shape);

            match direction {
                RoadDirection::BOTH => {
                    let id0 =
                        network.add_edge(shape_counter, node_start_id, node_end_id, shape_distance);
                    let id1 =
                        network.add_edge(shape_counter, node_end_id, node_start_id, shape_distance);

                    network.nodes.outgoing_edges[node_start_id].push(id0);
                    network.nodes.incoming_edges[node_end_id].push(id0);

                    network.nodes.outgoing_edges[node_end_id].push(id1);
                    network.nodes.incoming_edges[node_start_id].push(id1);
                }
                RoadDirection::WITH => {
                    let id0 =
                        network.add_edge(shape_counter, node_start_id, node_end_id, shape_distance);
                    network.nodes.incoming_edges[node_end_id].push(id0);
                    network.nodes.outgoing_edges[node_start_id].push(id0);
                }
                RoadDirection::AGAINST => {
                    let id1 =
                        network.add_edge(shape_counter, node_end_id, node_start_id, shape_distance);
                    network.nodes.outgoing_edges[node_end_id].push(id1);
                    network.nodes.incoming_edges[node_start_id].push(id1);
                }
            }
            shape_counter += 1;
        }

        let nodes = &network.nodes;
        assert_eq!(nodes.junctions.len(), nodes.outgoing_edges.len());
        assert_eq!(nodes.junctions.len(), nodes.incoming_edges.len());
        assert_eq!(nodes.junctions.len(), nodes.coordinate.len());

        let edges = &network.edges;
        assert_eq!(edges.object_id.len(), edges.source.len());
        assert_eq!(edges.object_id.len(), edges.target.len());
        assert_eq!(edges.object_id.len(), edges.distance.len());

        network
    }
}
