use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::network::{EdgeId, LiteNetwork, Network, NodeId};

use super::{EdgePath, ManyManyErrors, ManyManyPathAlgorithm};

pub struct ManyDijkstras {
    network: LiteNetwork,
}

impl ManyDijkstras {
    pub fn new(network: LiteNetwork) -> Self {
        Self { network }
    }

    pub fn network(&self) -> &LiteNetwork {
        &self.network
    }
}

impl ManyManyPathAlgorithm for ManyDijkstras {
    type Network = LiteNetwork;

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

        while !requests_pairs.is_empty() {
            let mut iterator = forward_propagation.iter_mut();
            let main = iterator.next().unwrap();
            if let Some((cost, _)) = main.next() {
                // println!("Cost: {}", cost);
                for f in iterator {
                    let l = f
                        .by_ref()
                        .take_while(|&(x, _)| x <= cost)
                        // .inspect(|x| println!("{:?}", x))
                        .count();
                    // println!("forward: {}", l);
                }
                for f in &mut backward_propagation {
                    let l = f
                        .by_ref()
                        // .inspect(|x| println!("{:?}", x))
                        .take_while(|&(x, _)| x <= cost)
                        .count();
                    // println!("backward: {}", l);
                }

                let mut found = Vec::new();

                for (i, j) in &requests_pairs {
                    let forward = forward_propagation[*i].visited();
                    let backward = backward_propagation[*j].visited();

                    if let Some(l) = forward
                        .keys()
                        .filter(|x| backward.contains_key(*x))
                        .min_by_key(|node| F32Wrapper(forward[node].0 + backward[node].0))
                    {
                        println!("found! {}", cost);
                        found.push((*i, *j, *l));
                    }
                }

                for (start, target, middle_node) in found {
                    requests_pairs.remove(&(start, target));

                    let mut part_1 = forward_propagation[start].rebuild(middle_node);
                    let mut part_2 = backward_propagation[target].rebuild(middle_node);

                    part_1.pop().unwrap();
                    part_1.append(&mut part_2);

                    println!(
                        "distance: {}",
                        part_1
                            .iter()
                            .map(|x| self.network.edge_distance(*x))
                            .sum::<f32>()
                    );

                    found_paths.push(EdgePath {
                        source: nodes[start],
                        target: nodes[target],
                        edges: part_1,
                    })
                }
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
    visited: HashMap<NodeId, (f32, Option<EdgeId>)>, // The visited node id -> (the current cost, where it came from)
    heap: BinaryHeap<Reverse<DijkstraIteratorEntry>>,
    direction: DijkstraDirection,
    network: &'a T,
}

impl<'a, T: Network> DijkstraIterator<'a, T> {
    pub fn new(network: &'a T, start: NodeId, direction: DijkstraDirection) -> Self {
        let mut initial_heap = BinaryHeap::new();
        let mut initial_map = HashMap::new();
        initial_heap.push(Reverse(DijkstraIteratorEntry {
            node: start,
            cost: 0f32,
        }));
        initial_map.insert(start, (0f32, None));
        DijkstraIterator {
            visited: initial_map,
            heap: initial_heap,
            direction,
            network,
        }
    }

    pub fn visited(&self) -> &HashMap<NodeId, (f32, Option<EdgeId>)> {
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
            println!(
                "node: {:?}, edge: {:?}, dir: {:?}",
                node, prev, self.direction
            );
        }

        if let DijkstraDirection::Forward = self.direction {
            edges.reverse();
        }

        return edges;
    }
}

impl<'a, T: Network> Iterator for DijkstraIterator<'a, T> {
    type Item = (f32, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self.heap.pop()?.0;

        for (neighbour, edge) in self.direction.neighbours(entry.node, self.network) {
            let cost = entry.cost + self.network.edge_distance(edge);
            debug_assert!(cost >= entry.cost);

            if let Some((prev_cost, _)) = self.visited.get(&neighbour) {
                if cost >= *prev_cost {
                    continue;
                }
            }
            self.visited.insert(neighbour, (cost, Some(edge)));

            self.heap.push(Reverse(DijkstraIteratorEntry {
                node: neighbour,
                cost,
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
    cost: f32,
}

impl PartialOrd for DijkstraIteratorEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Ord for DijkstraIteratorEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.partial_cmp(&other.cost).unwrap()
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

#[derive(Debug, PartialEq, PartialOrd)]
struct F32Wrapper(f32);

impl Ord for F32Wrapper {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for F32Wrapper {}
