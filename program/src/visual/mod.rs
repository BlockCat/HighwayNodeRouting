use ggez::graphics::{self, BlendMode, Color, DrawMode, Drawable, FillOptions, MeshBuilder};
use shapefile::Polyline;

pub mod camera;
pub mod camera2;

#[derive(Debug)]
pub struct PolyLineWrapper<'a> {
    shape: &'a Polyline,
    color: Color,
    width: f32,
    blend: Option<BlendMode>,
}

impl<'a> PolyLineWrapper<'a> {
    pub fn new(shape: &'a Polyline, color: Color, width: f32) -> Self {
        Self { shape, blend: None, color, width }
    }
}

impl<'a> Drawable for PolyLineWrapper<'a> {
    fn draw(&self, ctx: &mut ggez::Context, param: ggez::graphics::DrawParam) -> ggez::GameResult {
        let poly = self.shape;

        let mut mb = MeshBuilder::new();

        for part in poly.parts() {
            let line = part
                .iter()
                .map(|point| [point.x as f32, point.y as f32])
                .collect::<Vec<[f32; 2]>>();

            mb.line(&line, self.width, self.color)?;
            let first = part.first().unwrap();
            let last = part.last().unwrap();
            for point in &[first, last] {
                mb.circle(
                    DrawMode::Fill(FillOptions::default()),
                    [point.x as f32, point.y as f32],
                    10.0f32,
                    1f32,
                    [0.3, 0.3, 0.3, 1.0].into(),
                );
            }
        }

        let mesh = mb.build(ctx)?;

        graphics::draw(ctx, &mesh, param)?;

        Ok(())
    }

    fn dimensions(&self, ctx: &mut ggez::Context) -> Option<ggez::graphics::Rect> {
        let poly = self.shape;
        let bbox = poly.bbox();
        Some(graphics::Rect::new(
            bbox.min.x as f32,
            bbox.min.y as f32,
            (bbox.max.x - bbox.min.x) as f32,
            (bbox.max.y - bbox.min.y) as f32,
        ))
    }

    fn set_blend_mode(&mut self, blend: Option<ggez::graphics::BlendMode>) {
        self.blend = blend;
    }

    fn blend_mode(&self) -> Option<ggez::graphics::BlendMode> {
        self.blend
    }
}
