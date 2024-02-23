use gl::TEXTURE_HEIGHT;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Rectangle2D {
    pos_x: i32,
    pos_y: i32,
    width: u32,
    height: u32,
    col: Color,
    object_type: DOType,
}

pub enum DOType {
    Pipe,
    Player,
}

impl Rectangle2D {
    pub fn new(
        pos_x: i32,
        pos_y: i32,
        width: u32,
        height: u32,
        col: Color,
        object_type: DOType,
    ) -> Rectangle2D {
        Rectangle2D {
            pos_x: pos_x,
            pos_y: pos_y,
            width: width,
            height: height,
            col: col,
            object_type: object_type,
        }
    }
}

pub trait DrawableObject {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>);
    fn get_pos_x(&self) -> i32;
    fn get_pos_y(&self) -> i32;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_col(&self) -> Color;
    fn get_object_type(&self) -> &DOType;

    fn set_pos_x(&mut self, pos_x: i32);
    fn set_pos_y(&mut self, pos_y: i32);
    fn set_width(&mut self, width: u32);
    fn set_height(&mut self, height: u32);
    fn set_col(&mut self, col: Color);
    fn set_object_type(&mut self, object_type: DOType);
}

impl DrawableObject for Rectangle2D {
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(self.col);
        let rect = Rect::new(self.pos_x, self.pos_y, self.width, self.height); // X, Y, Width, Height
        canvas.fill_rect(rect).expect("Failed to draw rectangle");
    }

    fn get_pos_x(&self) -> i32 {
        self.pos_x
    }
    fn get_pos_y(&self) -> i32 {
        self.pos_y
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_col(&self) -> Color {
        self.col
    }
    fn get_object_type(&self) -> &DOType {
        &self.object_type
    }

    fn set_pos_x(&mut self, pos_x: i32) {
        self.pos_x = pos_x;
    }
    fn set_pos_y(&mut self, pos_y: i32) {
        self.pos_y = pos_y;
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    fn set_col(&mut self, col: Color) {
        self.col = col;
    }
    fn set_object_type(&mut self, object_type: DOType) {
        self.object_type = object_type
    }
}
