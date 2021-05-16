use network::{LiteNetwork, Network, NodeCoord, NodeId};
use rand::{distributions::Uniform, prelude::StdRng, SeedableRng};
use std::time::SystemTime;

use crate::algorithm::{dijkstra::DijkstraPathAlgorithm, many_to_many_paths};

mod algorithm;
mod network;
mod preprocess;

fn main() {
    let network: LiteNetwork = preprocess::preprocess().expect("could not create/laod network");
    let nodes = random_nodes(147, StdRng::seed_from_u64(1), &network);

    let mut in2 = 0;
    let mut out1 = 0;
    let mut out2 = 0;

    for n in 0..network.nodes_len() {
        let o = network.outgoing_edges(NodeId(n)).len();
        let i = network.incoming_edges(NodeId(n)).len();

        if i == 1 && o == 2 {
            out2 += 1;
        }
        if i == 1 && o == 1 {
            out1 += 1;
        }
        if i == 2 && o == 1 {
            in2 += 1;
        }
    }

    println!("Total nodes: {}", network.nodes_len());
    println!(
        "Bypassable nodes: {}, [in2: {}, out1: {}, out2: {}]",
        in2 + out1 + out2,
        in2,
        out1,
        out2
    );

    let start = SystemTime::now();
    many_to_many_paths::<LiteNetwork, DijkstraPathAlgorithm>(&nodes, network).unwrap();
    let end = SystemTime::now();

    println!("Duration: {:?}", end.duration_since(start));
}

fn closest_node<S: Network>(network: &S, coord: NodeCoord) -> NodeId {
    (0..network.nodes_len())
        .map(|x| NodeId(x))
        .min_by_key(|x| Fwr(coord.distance_squared(&network.node_location(*x))))
        .unwrap()
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Fwr(f32);

impl Eq for Fwr {}

impl Ord for Fwr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn random_nodes<R: rand::Rng, N: Network>(size: usize, rnd: R, network: &N) -> Vec<NodeId> {
    rnd.sample_iter(Uniform::new(0, network.nodes_len()))
        .map(|id| NodeId(id))
        .take(size)
        .collect()
}

#[allow(dead_code)]
mod play {
    use std::collections::HashMap;

    use crate::{
        algorithm::{
            dijkstra::DijkstraPathAlgorithm, dijkstra_bi_dir::BiDirDijkstraPathAlgorithm,
            many_to_many_paths, EdgePath,
        },
        closest_node,
        network::{LiteNetwork, Network, NodeCoord, NodeId},
        preprocess,
    };

    const ZOETERMEER: NodeCoord = NodeCoord {
        x: 91467.0,
        y: 451279.0,
    };
    const UTRECHT: NodeCoord = NodeCoord {
        x: 137410.0,
        y: 452755.0,
    };

    const UTRECHT_2: NodeCoord = NodeCoord {
        x: 135644.0,
        y: 456549.0,
    };

    const NEUDE: NodeCoord = NodeCoord {
        x: 136482.0,
        y: 456166.0,
    };

    const UITHOF: NodeCoord = NodeCoord {
        x: 139791.0,
        y: 455493.0,
    };

    const BERGEN: NodeCoord = NodeCoord {
        x: 108286.0,
        y: 520286.0,
    };

    const HOUTEN: NodeCoord = NodeCoord {
        x: 139934.0,
        y: 449810.0,
    };

    fn play() {
        let network: LiteNetwork = preprocess::preprocess().expect("could not create/load network");
        println!("Nodes: {}", network.nodes_len());
        println!("Edges: {}", network.edge_len());

        let zoetermeer = closest_node(&network, ZOETERMEER);
        let utrecht = closest_node(&network, UTRECHT);
        let utrecht_2 = closest_node(&network, UTRECHT_2);
        let neude = closest_node(&network, NEUDE);
        let uithof = closest_node(&network, UITHOF);
        let bergen = closest_node(&network, BERGEN);
        let houten = closest_node(&network, HOUTEN);

        let mut map = HashMap::new();
        map.insert(zoetermeer, "zoetermeer");
        map.insert(utrecht, "utrecht");
        map.insert(utrecht_2, "utrecht_2");
        map.insert(neude, "neude");
        map.insert(uithof, "uithof");
        map.insert(bergen, "bergen");
        map.insert(houten, "houten");

        println!(
            "Distance between Zoetermeer and Utrecht: {}",
            network
                .node_location(zoetermeer)
                .distance(&network.node_location(utrecht))
        );

        // visualise_path(zoetermeer, utrecht, network).unwrap();
        let nodes = &[
            zoetermeer, utrecht, utrecht_2, neude, uithof, bergen, houten,
        ];
        let res = many_to_many_paths::<LiteNetwork, DijkstraPathAlgorithm>(nodes, network.clone());
        print_csv(map.clone(), nodes, res.unwrap(), &network);
        let res =
            many_to_many_paths::<LiteNetwork, BiDirDijkstraPathAlgorithm>(nodes, network.clone());
        print_csv(map, nodes, res.unwrap(), &network);
    }

    fn print_csv(
        map: HashMap<NodeId, &str>,
        nodes: &[NodeId],
        paths: Vec<EdgePath>,
        network: &LiteNetwork,
    ) {
        print!(",");
        for n in 0..nodes.len() {
            print!("{},", map[&nodes[n]]);
        }

        let mapping = paths
            .iter()
            .map(|x| {
                (
                    (x.source, x.target),
                    x.edges
                        .iter()
                        .map(|x| network.edge_distance(*x))
                        .sum::<f32>(),
                )
            })
            .collect::<HashMap<_, _>>();

        println!();
        for a in (0..nodes.len()).map(|x| nodes[x]) {
            print!("{},", map[&a]);
            for b in (0..nodes.len()).map(|x| nodes[x]) {
                if a == b {
                    print!(",");
                } else {
                    print!("{},", mapping[&(a, b)]);
                }
            }
            println!();
        }
    }
}
