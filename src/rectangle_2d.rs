use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Rectangle2D {
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: u32,
    pub height: u32,
    pub col: Color,
}

pub trait Drawable {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
}

impl Drawable for Rectangle2D {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.col);
        let rect = Rect::new(self.pos_x, self.pos_y, self.width, self.height); // X, Y, Width, Height
        canvas.fill_rect(rect).expect("Failed to draw rectangle");
    }
}
