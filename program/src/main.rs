use algorithm::{simple_a_star::SimpleAStar, EdgePath, PathAlgorithm};
use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self},
    nalgebra::Point2,
    Context, GameResult,
};
use network::{EdgeId, LiteNetwork, Network, NodeCoord, NodeId};
use shapefile::{Error, Polyline};
use std::{collections::HashMap, path::Path, time::SystemTime};
use visual::{camera, camera2::Camera};

use crate::algorithm::{ManyToManyAlgorithm, dijkstra::DijkstraPathAlgorithm, dijkstra_bi_dir::BiDirDijkstraPathAlgorithm};

mod algorithm;
mod network;
mod preprocess;
mod visual;

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

struct MainState {
    // image: Image,
    shapes: Vec<Polyline>,
    camera: Camera,
    path: Vec<EdgeId>,
    network: LiteNetwork,
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        camera::draw(ctx, &self.shapes, &self.camera, &self.path, &self.network)?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Up => self.camera.move_by(Point2::new(0.0, 500.0)),
            event::KeyCode::Left => self.camera.move_by(Point2::new(-500.0, 0.0)),
            event::KeyCode::Down => self.camera.move_by(Point2::new(0.0, -500.0)),
            event::KeyCode::Right => self.camera.move_by(Point2::new(500.0, 0.0)),
            _ => (),
        };

        println!("{:?}", self.camera);
    }
}

fn visualise_path(node_a: NodeId, node_b: NodeId, network: LiteNetwork) -> GameResult {
    let algorithm = SimpleAStar::new(network.clone());

    let path = if let Ok((_, path)) = algorithm.path(node_a, node_b) {
        let distance = path
            .iter()
            .map(|x| algorithm.network().edge_distance(*x))
            .sum::<f32>();
        println!("Path found with {} edges", path.len());
        println!("Path has a distance of: {}", distance);

        println!("{:?}", path);
        path
    } else {
        println!("No path found");
        Vec::new()
    };

    let shapes = read("data/Wegvakken/Wegvakken.shp").unwrap();

    // shapes.truncate(10);

    let mut camera = Camera::new(1000, 1000, 20_000f32, 20_000f32);
    camera.move_to([93967.0, 459279.0].into());
    let mut state = MainState {
        shapes,
        camera,
        path,
        network: network,
    };

    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0));
    let (mut ctx, mut event_loop) = cb.build()?;
    ggez::event::run(&mut ctx, &mut event_loop, &mut state)
}

fn dijkstra_many_to_many<T: ManyToManyAlgorithm<Network = LiteNetwork>>(nodes: &[NodeId], network: LiteNetwork) -> Vec<EdgePath> {
    let algorithm = T::new(network);

    println!("Searching {} paths", nodes.len() * (nodes.len() - 1));

    let start = SystemTime::now();
    let result = algorithm.path(nodes);
    let end = SystemTime::now();

    println!("Time: {:?}", end.duration_since(start));

    if let Ok(result) = result {
        println!("Found {} paths", result.len());

        for res in &result {
            let distance = res
                .edges
                .iter()
                .map(|x| algorithm.network().edge_distance(*x))
                .sum::<f32>();

            println!("Distance: {}", distance);
        }
        result
    } else {
        println!("No paths found or some other error: {:?}", result);
        Vec::new()
    }
}
fn main() {
    let network: LiteNetwork = preprocess::preprocess().expect("could not create/laod network");
    println!("Nodes: {}", network.node_len());
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
    let res = dijkstra_many_to_many::<DijkstraPathAlgorithm>(nodes, network.clone());
    print_csv(map.clone(), nodes, res, &network);
    let res = dijkstra_many_to_many::<BiDirDijkstraPathAlgorithm>(nodes, network.clone());
    print_csv(map, nodes, res, &network);
}

fn read<P: AsRef<Path>>(path: P) -> Result<Vec<Polyline>, Error> {
    //Result<Vec<(Polyline, HashMap<String, FieldValue>)>, Error> {
    let reader = shapefile::Reader::from_path(path.as_ref())?;
    let iter = reader.iter_shapes_as::<Polyline>();

    println!(
        "{:?}",
        shapefile::Reader::from_path(path.as_ref())?
            .iter_shapes_and_records_as::<Polyline>()?
            .next()
    );

    iter.collect()
}

fn closest_node<S: Network>(network: &S, coord: NodeCoord) -> NodeId {
    (0..network.nodes_len())
        .map(|x| NodeId(x))
        .min_by_key(|x| Fwr(coord.distance_squared(&network.node_location(*x))))
        .unwrap()
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

#[derive(Debug, PartialOrd, PartialEq)]
struct Fwr(f32);

impl Eq for Fwr {}

impl Ord for Fwr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
