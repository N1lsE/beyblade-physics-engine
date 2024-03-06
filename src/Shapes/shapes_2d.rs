use super::traits::{Clickable, Drawable, Shape};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub struct Rectangle {
    rect: Rect,
    color: Color,
    is_filled: bool,
}

impl Shape for Rectangle {}

impl Drawable for Rectangle {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.color);

        if self.is_filled {
            canvas.fill_rect(self.rect)?;
        } else {
            canvas.draw_rect(self.rect)?;
        }
        Ok(())
    }
}

impl Rectangle {
    pub fn new(rect: Rect, color: Color, is_filled: bool) -> Rectangle {
        Rectangle {
            rect,
            color,
            is_filled,
        }
    }
}

pub struct Button {
    text: String,
    rect: Rect,
    background_color: Color,
    text_color: Color,
    is_filled: bool,
    click_event: Box<dyn Fn() -> ()>,
}

impl Shape for Button {}
impl Clickable for Button {
    fn on_click(&self) -> Result<(), String> {
        (self.click_event)();
        Ok(())
    }
    fn is_clicked(&self, x: i32, y: i32) -> bool {
        if x >= self.rect.x
            && x <= self.rect.x + self.rect.width() as i32
            && y >= self.rect.y
            && y <= self.rect.y + self.rect.height() as i32
        {
            return true;
        }
        return false;
    }
}
impl Drawable for Button {
    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        canvas.set_draw_color(self.background_color);

        if self.is_filled {
            canvas.fill_rect(self.rect)?;
        } else {
            canvas.draw_rect(self.rect)?;
        }
        Ok(())
    }
}

impl Button {
    pub fn new(
        text: String,
        rect: Rect,
        background_color: Color,
        text_color: Color,
        is_filled: bool,
        click_event: Box<dyn Fn()>,
    ) -> Button {
        Button {
            text,
            rect,
            background_color,
            text_color,
            is_filled,
            click_event,
        }
    }
}
