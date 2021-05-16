use super::{EdgePath, ManyManyErrors, ManyToManyAlgorithm};
use crate::{
    algorithm::dijkstra::{DijkstraDirection, DijkstraIterator},
    network::{LiteNetwork, Network, NodeId},
};
use std::collections::HashSet;

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

    fn path(&self, nodes: &[NodeId]) -> Result<Vec<super::EdgePath>, ManyManyErrors> {
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
