use std::collections::HashMap;

use crate::visual::PolyLineWrapper;
use ggez::{filesystem::print_all, Context};
use ggez::{graphics, GameResult};
use graphics::DrawParam;
use mint::Vector2;
use shapefile::{dbase::FieldValue, Polyline, Shape};

use super::camera2::Camera;
use super::camera2::CameraDraw;

pub fn draw(context: &mut Context, shapes: &Vec<(Polyline)>, camera: &Camera) -> GameResult<()> {
    println!("s: {}", shapes.len());
    let r = camera.rect();
    let mut sum = 0usize;

    for (shape) in shapes.iter() {
        let poly = PolyLineWrapper::new(shape);

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
