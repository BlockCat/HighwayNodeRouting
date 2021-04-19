use std::collections::HashSet;

use super::camera2::Camera;
use crate::{
    network::{EdgeId, LiteNetwork, Network},
    visual::PolyLineWrapper,
};
use ggez::Context;
use ggez::{graphics, GameResult};
use shapefile::Polyline;

pub fn draw(
    context: &mut Context,
    shapes: &Vec<Polyline>,
    camera: &Camera,
    path: &Vec<EdgeId>,
    network: &LiteNetwork,
) -> GameResult<()> {
    println!("s: {}", shapes.len());
    let r = camera.rect();
    let mut sum = 0usize;

    let path = path
        .iter()
        .map(|x| network.edge_object_id(*x))
        .collect::<HashSet<_>>();

    for (id, shape) in shapes.iter().enumerate() {
        let poly = if path.contains(&id) {
            PolyLineWrapper::new(shape, [0.0, 1.0, 0.0, 1.0].into(), 30.0)
        } else {
            PolyLineWrapper::new(shape, [1.0, 1.0, 1.0, 1.0].into(), 10.0)
        };

        let d = poly
            .shape
            .parts()
            .iter()
            .flat_map(|x| x.iter())
            .any(|x| r.contains([x.x as f32, x.y as f32]));
        if d {
            graphics::draw(context, &poly, camera.clone())?;
            sum += 1;
        }
    }

    println!("Drawn: {}", sum);

    Ok(())
}
