//! A camera object for ggez.
//! Currently ggez has no actual global camera state to use,
//! so this really just does the coordinate transforms for you.
//!
//! Basically it translates ggez's coordinate system with the origin
//! at the top-left and Y increasing downward to a coordinate system
//! with the origin at the center of the screen and Y increasing
//! upward.
//!
//! Because that makes sense, darn it.
//!
//! However, does not yet do any actual camera movements like
//! easing, pinning, etc.
//! But a great source for how such things work is this:
//! http://www.gamasutra.com/blogs/ItayKeren/20150511/243083/Scroll_Back_The_Theory_and_Practice_of_Cameras_in_SideScrollers.php

// TODO: Debug functions to draw world and camera grid!

use ggez::graphics::{self, DrawParam, Drawable, Rect};
use ggez::{self, nalgebra::Point2};

use ggez::GameResult;

type Vec2 = Point2<f32>;

// Hmm.  Could, instead, use a 2d transformation
// matrix, or create one of such.
#[derive(Debug, Clone)]
pub struct Camera {
    screen_size: Vec2,
    view_size: Vec2,
    view_center: Vec2,
}

impl Camera {
    pub fn new(screen_width: u32, screen_height: u32, view_width: f32, view_height: f32) -> Self {
        let screen_size = Vec2::new(screen_width as f32, screen_height as f32);
        let view_size = Vec2::new(view_width as f32, view_height as f32);
        Camera {
            screen_size: screen_size,
            view_size: view_size,
            view_center: Vec2::new(0.0, 0.0),
        }
    }

    pub fn move_by(&mut self, by: Vec2) {
        self.view_center.x += by.x;
        self.view_center.y += by.y;
    }

    pub fn move_to(&mut self, to: Vec2) {
        self.view_center = to;
    }

    /// Translates a point in world-space to a point in
    /// screen-space.
    ///
    /// Does not do any clipping or anything, since it does
    /// not know how large the thing that might be drawn is;
    /// that's not its job.
    pub fn world_to_screen_coords(&self, from: Vec2) -> (i32, i32) {
        let pixels_per_unit = Point2::new(
            self.screen_size.x / self.view_size.x,
            self.screen_size.y / self.view_size.y,
        );
        let view_offset = from - self.view_center;
        let view_scale = Point2::new(
            view_offset.x * pixels_per_unit.x,
            view_offset.y * pixels_per_unit.y,
        );

        let x = view_scale.x + self.screen_size.x / 2.0;
        let y = self.screen_size.y - (view_scale.y + self.screen_size.y / 2.0);
        (x as i32, y as i32)
    }

    // p_screen = max_p - p + max_p/2
    // p_screen - max_p/2 = max_p - p
    // p_screen - max_p/2 + max_p = -p
    // -p_screen - max_p/2 + max_p = p
    pub fn screen_to_world_coords(&self, from: (i32, i32)) -> Vec2 {
        let (sx, sy) = from;
        let sx = sx as f32;
        let sy = sy as f32;
        let flipped_x = sx - (self.screen_size.x / 2.0);
        let flipped_y = -sy + self.screen_size.y / 2.0;
        let screen_coords = Vec2::new(flipped_x, flipped_y);
        let units_per_pixel = Point2::new(
            self.view_size.x * self.screen_size.x,
            self.view_size.y * self.screen_size.y,
        );
        let view_scale = Point2::new(
            screen_coords.x * units_per_pixel.x,
            screen_coords.y * units_per_pixel.y,
        );
        let view_offset = Point2::new(
            self.view_center.x + view_scale.x,
            self.view_center.y + view_scale.y,
        );

        view_offset
    }

    pub fn rect(&self) -> Rect {
        let l = self.view_center - self.view_size / 2f32;
        Rect::new(l.x, l.y - self.view_size.y, self.view_size.x, self.view_size.y)
    }

    pub fn location(&self) -> Vec2 {
        self.view_center
    }

    fn calculate_dest_point(&self, location: Vec2) -> Vec2 {
        let (sx, sy) = self.world_to_screen_coords(location);
        Vec2::new(sx as f32, -sy as f32)
    }
}

pub trait CameraDraw
where
    Self: graphics::Drawable,
{
    fn draw_ex_camera(
        &self,
        camera: &Camera,
        ctx: &mut ggez::Context,
        p: ggez::graphics::DrawParam,
    ) -> GameResult<()> {
        let dest = camera.calculate_dest_point(Point2::new(p.dest.x, p.dest.y));
        self.draw(ctx, p.dest(dest))
    }

    fn draw_camera(
        &self,
        camera: &Camera,
        ctx: &mut ggez::Context,
        dest: Vec2,
        rotation: f32,
    ) -> GameResult<()> {
        let dest = camera.calculate_dest_point(dest);
        self.draw(
            ctx,
            ggez::graphics::DrawParam::default()
                .dest(dest)
                .rotation(rotation),
        )
    }
}

impl<T> CameraDraw for T where T: graphics::Drawable {}

impl Into<DrawParam> for Camera {
    fn into(self) -> DrawParam {
        let pixels_per_unit = Point2::new(
            self.screen_size.x / self.view_size.x,
            self.screen_size.y / self.view_size.y,
        );
        let view_offset = self.view_center - self.view_size / 2f32;

        DrawParam::new()
            .dest(Point2::new(-view_offset.x, -view_offset.y))
            .offset(Point2::new(view_offset.x, view_offset.y))            
            .scale([pixels_per_unit.x, -pixels_per_unit.y])
    }
}
