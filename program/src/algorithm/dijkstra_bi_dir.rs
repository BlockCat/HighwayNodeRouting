use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::network::{EdgeId, LiteNetwork, Network, NodeId};

use super::{EdgePath, ManyManyErrors, ManyToManyAlgorithm};

pub struct BiDirDijkstraPathAlgorithm {
    network: LiteNetwork,
}

impl ManyToManyAlgorithm for BiDirDijkstraPathAlgorithm {
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
        let mut backward_propagation = nodes
            .iter()
            .map(|node| DijkstraIterator::new(&self.network, *node, DijkstraDirection::Backward))
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
        let mut nodes_evaluated = 0;

        let mut cost_range = 0;

        while !requests_pairs.is_empty() {
            cost_range += 100;
            for f in &mut forward_propagation {
                let pc = f.peek_cost();
                if pc.is_some() && pc.unwrap() <= cost_range {
                    nodes_evaluated += f.by_ref().take_while(|&(x, _)| x <= cost_range).count();
                }
                assert!(f.peek_cost().unwrap() > cost_range);
            }
            for f in &mut backward_propagation {
                let pc = f.peek_cost();
                if pc.is_some() && pc.unwrap() <= cost_range {
                    nodes_evaluated += f.by_ref().take_while(|&(x, _)| x <= cost_range).count();
                }
                assert!(f.peek_cost().unwrap() > cost_range);
            }

            let mut found = Vec::new();

            for (i, j) in &requests_pairs {
                let forward = forward_propagation[*i].visited();
                let backward = backward_propagation[*j].visited();

                let intersected = forward
                    .keys()
                    .filter(|x| backward.contains_key(*x))
                    .collect::<Vec<_>>();

                let len = intersected.len();

                if let Some(l) = intersected
                    .into_iter()
                    .min_by_key(|node| (forward[node].0 + backward[node].0))
                {
                    println!(
                        "found at range {}, cost: {}, others: {}",
                        cost_range,
                        forward[l].0 + backward[l].0,
                        len
                    );
                    found.push((*i, *j, *l));
                }
            }

            for (start, target, middle_node) in found {
                requests_pairs.remove(&(start, target));

                let mut part_1 = forward_propagation[start].rebuild(middle_node);
                let mut part_2 = backward_propagation[target].rebuild(middle_node);

                part_1.append(&mut part_2);

                println!(
                    "distance: {}",
                    part_1
                        .iter()
                        .map(|x| self.network.edge_distance(*x) as usize)
                        .sum::<usize>()
                );

                found_paths.push(EdgePath {
                    source: nodes[start],
                    target: nodes[target],
                    edges: part_1,
                })
            }
        }

        // All pairs should be found, excluding path to own.
        println!("Nodes evaluated: {}", nodes_evaluated);
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

    fn peek_cost(&self) -> Option<usize> {
        self.heap.peek().map(|x| x.0.cost)
    }

    pub fn visited(&self) -> &HashMap<NodeId, (usize, Option<EdgeId>)> {
        &self.visited
    }

    pub fn rebuild(&self, mut node: NodeId) -> Vec<EdgeId> {
        let mut edges = Vec::new();
        while let Some((_, Some(prev))) = &self.visited.get(&node) {
            edges.push(*prev);
            node = match self.direction {
                DijkstraDirection::Forward => self.network.edge_source(*prev),
                DijkstraDirection::Backward => self.network.edge_target(*prev),
            };
            // println!(
            //     "node: {:?}, edge: {:?}, dir: {:?}",
            //     node, prev, self.direction
            // );
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

        // println!("{:?}", self.heap);
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
enum DijkstraDirection {
    Forward,
    Backward,
}

impl DijkstraDirection {
    fn neighbours<T: Network>(&self, node: NodeId, network: &T) -> Vec<(NodeId, EdgeId)> {
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

// #[derive(Debug, PartialEq, PartialOrd)]
// struct F32Wrapper(f32);

// impl Ord for F32Wrapper {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0.partial_cmp(&other.0).unwrap()
//     }
// }

// impl Eq for F32Wrapper {}
