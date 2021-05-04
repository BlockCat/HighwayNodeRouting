use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::network::{EdgeId, LiteNetwork, Network, NodeId};

use super::{EdgePath, ManyManyErrors, ManyToManyAlgorithm};

pub struct DijkstraPathAlgorithm {
    network: LiteNetwork,
}

impl ManyToManyAlgorithm for DijkstraPathAlgorithm {
    type Network = LiteNetwork;

    fn new(network: Self::Network) -> Self {
        Self { network }
    }

    fn network(&self) -> &Self::Network {
        &self.network
    }

    fn path(
        &self,
        nodes: &[crate::network::NodeId],
    ) -> Result<Vec<super::EdgePath>, ManyManyErrors> {
        if nodes.is_empty() {
            return Err(ManyManyErrors::EmptyNodeList);
        }

        let mut forward_propagation = nodes
            .iter()
            .map(|node| DijkstraIterator::new(&self.network, *node, DijkstraDirection::Forward))
            .collect::<Vec<_>>();

        let mut requests_pairs = HashSet::new();

        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if i != j {
                    requests_pairs.insert((i, j));
                }
            }
        }

        let mut found_paths = Vec::new();

        for i in 0..nodes.len() {
            let node = &nodes[i];
            let prop = &mut forward_propagation[i];
            let mut set = nodes.iter().collect::<HashSet<_>>();
            set.remove(node);
            for x in prop.by_ref() {
                if set.remove(&x.1) && set.is_empty() {
                    break;
                }
            }

            println!("{:?}", node);

            for j in 0..nodes.len() {
                if i == j {
                    continue;
                }

                found_paths.push(EdgePath {
                    source: nodes[i],
                    target: nodes[j],
                    edges: prop.rebuild(nodes[j]),
                });
            }
        }

        // All pairs should be found, excluding path to own.
        if found_paths.len() == nodes.len() * (nodes.len() - 1) {
            Ok(found_paths)
        } else {
            Err(ManyManyErrors::NotAllPairsFound(found_paths))
        }
    }
}

struct DijkstraIterator<'a, T: Network> {
    visited: HashMap<NodeId, (usize, Option<EdgeId>)>, // The visited node id -> (the current cost, where it came from)
    heap: BinaryHeap<Reverse<DijkstraIteratorEntry>>,
    direction: DijkstraDirection,
    network: &'a T,
}

impl<'a, T: Network> DijkstraIterator<'a, T> {
    pub fn new(network: &'a T, start: NodeId, direction: DijkstraDirection) -> Self {
        let mut initial_heap = BinaryHeap::new();

        initial_heap.push(Reverse(DijkstraIteratorEntry {
            node: start,
            cost: 0,
            edge: None,
        }));
        // initial_map.insert(start, (0f32, None));
        DijkstraIterator {
            visited: HashMap::new(),
            heap: initial_heap,
            direction,
            network,
        }
    }

    pub fn rebuild(&self, mut node: NodeId) -> Vec<EdgeId> {
        let mut edges = Vec::new();
        while let Some((_, Some(prev))) = &self.visited.get(&node) {
            edges.push(*prev);
            node = match self.direction {
                DijkstraDirection::Forward => self.network.edge_source(*prev),
                DijkstraDirection::Backward => self.network.edge_target(*prev),
            };
        }

        if let DijkstraDirection::Forward = self.direction {
            edges.reverse();
        }

        return edges;
    }
}

impl<'a, T: Network> Iterator for DijkstraIterator<'a, T> {
    type Item = (usize, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry = self.heap.pop()?.0;
        while let Some((prev_cost, _)) = self.visited.get(&entry.node) {
            if entry.cost >= *prev_cost {
                entry = self.heap.pop()?.0;
            } else {
                break;
            }
        }

        self.visited.insert(entry.node, (entry.cost, entry.edge));

        for (neighbour, edge) in self.direction.neighbours(entry.node, self.network) {
            let cost = entry.cost + self.network.edge_distance(edge) as usize;
            debug_assert!(cost >= entry.cost);

            self.heap.push(Reverse(DijkstraIteratorEntry {
                node: neighbour,
                cost: cost,
                edge: Some(edge),
            }));
        }

        debug_assert!(
            self.heap.peek().unwrap().0.cost >= entry.cost,
            "{} > {}",
            self.heap.peek().unwrap().0.cost,
            entry.cost
        );
        Some((entry.cost, entry.node))
    }
}

#[derive(Debug, PartialEq)]
struct DijkstraIteratorEntry {
    node: NodeId,
    cost: usize,
    edge: Option<EdgeId>,
}

impl PartialOrd for DijkstraIteratorEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost
            .partial_cmp(&other.cost)
            .map(|x| x.then(self.node.0.cmp(&other.node.0)))
    }
}

impl Ord for DijkstraIteratorEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost
            .partial_cmp(&other.cost)
            .unwrap()
            .then(self.node.0.cmp(&other.node.0))
    }
}

impl Eq for DijkstraIteratorEntry {}

#[derive(Debug)]
pub enum DijkstraDirection {
    Forward,
    Backward,
}

impl DijkstraDirection {
    pub fn neighbours<T: Network>(&self, node: NodeId, network: &T) -> Vec<(NodeId, EdgeId)> {
        match self {
            DijkstraDirection::Forward => network
                .outgoing_edges(node)
                .iter()
                .map(|edge| (network.edge_target(*edge), *edge))
                .collect::<Vec<_>>(),
            DijkstraDirection::Backward => network
                .incoming_edges(node)
                .iter()
                .map(|edge| (network.edge_source(*edge), *edge))
                .collect::<Vec<_>>(),
        }
    }
}
