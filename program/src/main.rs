use std::collections::HashMap;

use ggez::{
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, MeshBuilder},
    nalgebra::Point2,
    Context, GameResult,
};
use shapefile::{dbase::FieldValue, Error, Polyline, Shape};
use visual::{camera, camera2::Camera};

mod visual;

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

fn main() -> GameResult {
    println!("Hello, world!");
    let mut shapes = read().unwrap();

    // shapes.truncate(10);

    let mut camera = Camera::new(1000, 1000, 10_000f32, 10_000f32);
    camera.move_to([139199.0, 459748.0f32].into());
    let mut state = MainState { shapes, camera };

    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 800.0));
    let (mut ctx, mut event_loop) = cb.build()?;
    ggez::event::run(&mut ctx, &mut event_loop, &mut state)
}

fn read() -> Result<Vec<Polyline>, Error> {
    //Result<Vec<(Polyline, HashMap<String, FieldValue>)>, Error> {
    let reader = shapefile::Reader::from_path("data/Wegvakken/Wegvakken.shp")?;
    let iter = reader.iter_shapes_as::<Polyline>();

    iter.collect()
}
