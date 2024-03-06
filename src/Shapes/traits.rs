use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Shape {}
pub trait Drawable {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
}
pub trait Clickable {
    fn on_click(&self) -> Result<(), String>;
    fn is_clicked(&self, x: i32, y: i32) -> bool;
}
