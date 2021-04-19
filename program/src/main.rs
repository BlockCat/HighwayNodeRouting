use std::{collections::HashMap, path::Path};

use algorithm::{simple_a_star::SimpleAStar, Algorithm};
use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, MeshBuilder},
    nalgebra::Point2,
    Context, GameResult,
};
use network::{AoSNetwork, LiteNetwork, Network, NodeCoord, NodeId};
use shapefile::{dbase::FieldValue, Error, Polyline, Shape};
use visual::{camera, camera2::Camera};

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

struct MainState {
    // image: Image,
    shapes: Vec<(Polyline)>,
    camera: Camera,
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        camera::draw(ctx, &self.shapes, &self.camera)?;

        let m = MeshBuilder::new()
            .line(
                &[[23199f32, 392248f32], [24199f32, 393248f32]],
                2f32,
                [0.0, 1.0, 0.0, 1.0].into(),
            )?
            .build(ctx)?;

        graphics::draw(ctx, &m, self.camera.clone())?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
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

fn main() {
    // -> GameResult {
    println!("Hello, world!");
    // let mut shapes = read("data/Wegvakken/Wegvakken.shp").unwrap();

    // // shapes.truncate(10);

    // let mut camera = Camera::new(1000, 1000, 10_000f32, 10_000f32);
    // camera.move_to([190895.0, 427154.0f32].into());
    // let mut state = MainState { shapes, camera };

    // let cb = ggez::ContextBuilder::new("super_simple", "ggez")
    //     .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0));
    // let (mut ctx, mut event_loop) = cb.build()?;
    // ggez::event::run(&mut ctx, &mut event_loop, &mut state)
    let network: LiteNetwork = preprocess::preprocess().expect("could not create/laod network");
    println!("Nodes: {}", network.node_len());
    println!("Edges: {}", network.edge_len());

    let zoetermeer = closest_node(&network, ZOETERMEER);
    let utrecht = closest_node(&network, UTRECHT);

    println!(
        "Distance between Zoetermeer and Utrecht: {}",
        network
            .node_location(zoetermeer)
            .distance(&network.node_location(utrecht))
    );
    // println!("Nodes: {}", network.nodes.len());
    // println!("Edges: {}", network.edges.len());

    let algorithm = SimpleAStar::new(network);

    if let Ok((_, path)) = algorithm.path(utrecht, zoetermeer) {
        let distance = path
            .iter()
            .map(|x| algorithm.network().edge_distance(*x))
            .sum::<f32>();
        println!("Path found with {} edges", path.len());
        println!("Path has a distance of: {}", distance);

        println!("{:?}", path);
    } else {
        println!("No path found");
    }

    // println!(
    //     "Total distance: {}",
    //     network
    //         .edges
    //         .iter()
    //         .flat_map(|x| x.iter().map(|e| e.distance))
    //         .sum::<f32>()
    // );
    // println!("{:?}",);
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

#[derive(Debug, PartialOrd, PartialEq)]
struct Fwr(f32);

impl Eq for Fwr {}

impl Ord for Fwr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}
