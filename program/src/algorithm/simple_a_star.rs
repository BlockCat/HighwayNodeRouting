use crate::network::{EdgeId, Network, NodeCoord, NodeId};
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::PathAlgorithm;

pub struct SimpleAStar<A>
where
    A: Network,
{
    network: A,
}

#[allow(dead_code)]
impl<A> SimpleAStar<A>
where
    A: Network,
{
    pub fn new(network: A) -> Self {
        SimpleAStar { network }
    }

    pub fn network(&self) -> &A {
        &self.network
    }
}

impl<A> PathAlgorithm for SimpleAStar<A>
where
    A: Network,
{
    type Network = A;
    type Output = ();

    fn path(
        &self,
        source: crate::network::NodeId,
        target: crate::network::NodeId,
    ) -> Result<(Self::Output, Vec<crate::network::EdgeId>), ()> {
        {
            let start_node = self.network.junction_id(source);
            let end_node = self.network.junction_id(target);
            let distance = self
                .network
                .node_location(source)
                .distance(&self.network.node_location(target));

            println!(
                "Finding path from: {}({}) towards {}({}), around: {} meters",
                start_node, source.0, end_node, target.0, distance
            );
        }

        let source_coord = self.network.node_location(source);
        let target_coord = self.network.node_location(target);

        let mut from: HashMap<NodeId, (f32, Option<EdgeId>)> = HashMap::new();
        from.insert(source, (0.0, None));

        let mut heap = BinaryHeap::new();

        heap.push(HeapEntry {
            cost: heuristic(0f32, source_coord, target_coord),
            distance: 0f32,
            node: source,
        });

        let mut visited = HashSet::new();

        while let Some(entry) = heap.pop() {
            if entry.node == target {
                let mut m = target;
                let mut edges = Vec::new();
                while let (_, Some(prev)) = from[&m] {
                    edges.push(prev);
                    let s = self.network.edge_source(prev);
                    m = s;
                }
                edges.reverse();
                return Ok(((), edges));
            }

            if !visited.insert(entry.node) {
                continue;
            }

            // println!("--");
            // println!("node: {:?}", entry);

            let children = self.network.outgoing_edges(entry.node);
            let distances = children
                .iter()
                .map(|x| self.network.edge_distance(*x))
                .collect::<Vec<_>>();
            let neighbours = children
                .iter()
                .map(|x| self.network.edge_target(*x))
                .collect::<Vec<_>>();
            let locations = neighbours
                .iter()
                .map(|x| self.network.node_location(*x))
                .collect::<Vec<_>>();

            for (child, edge) in neighbours
                .into_iter()
                .zip(locations.into_iter())
                .zip(distances.into_iter())
                .zip(children.iter())
                .map(|(((node, coords), distance), edge)| {
                    (
                        HeapEntry {
                            node,
                            distance: entry.distance + distance,
                            cost: heuristic(entry.distance + distance, coords, target_coord),
                        },
                        *edge,
                    )
                })
            {
                // println!("child: {:?}", child);
                let ndist = entry.distance + self.network.edge_distance(edge);
                if let Some((d, x)) = from.get_mut(&child.node) {
                    if ndist < *d {
                        *d = ndist;
                        *x = Some(edge);
                    }
                } else {
                    from.insert(child.node, (ndist, Some(edge)));
                }
                heap.push(child);
            }
        }

        Err(())
    }
}

fn heuristic(cost: f32, source: NodeCoord, target: NodeCoord) -> f32 {
    // cost + source.distance(&target)
    -(cost + source.distance(&target))
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct HeapEntry {
    cost: f32,
    distance: f32,
    node: NodeId,
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.partial_cmp(&other.cost).unwrap()
    }
}

impl Eq for HeapEntry {}
