use crate::winsdl::Winsdl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::{Duration, Instant};

mod rectangle_2d;
use rectangle_2d::{Drawable, Rectangle2D};
mod winsdl;

fn main() {
    let TARGET_FPS: u32 = 60;
    let FRAME_TIME: Duration = Duration::from_secs(1) / TARGET_FPS;
    let SCREEN_WIDTH = 800;
    let SCREEN_HEIGHT = 600;

    let mut winsdl = Winsdl::new(SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();

    let mut canvas = winsdl.window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut background = texture_creator
        .create_texture_target(
            sdl2::pixels::PixelFormatEnum::RGBA8888,
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    let mut last_frame_time = Instant::now();

    let mut drawable: Vec<Box<dyn Drawable>> = Vec::new();

    'running: loop {
        let frame_start = Instant::now();

        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // let time_elapsed = (start.elapsed().as_secs_f32().sin() + 1.0) / 2.0;
        // println!("{}", time_elapsed);

        // Clear Canvas
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Draw on Canvas
        canvas
            .with_texture_canvas(&mut background, |texture_canvas| {
                draw(texture_canvas);
            })
            .unwrap();

        // Kopiere Hintergrundpuffer auf Vordergrundpuffer
        canvas.copy(&background, None, None).unwrap();

        // Render Canvas
        canvas.present();

        // Limit FPS
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_TIME {
            std::thread::sleep(FRAME_TIME - elapsed);
        }

        last_frame_time = frame_start;
    }
}

fn draw(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, drawable: Vec<Box<dyn Drawable>>) {
    // Draw Background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Draw Rect
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let rect = Rect::new(100, 100, 200, 150); // X, Y, Width, Height
    canvas.fill_rect(rect).expect("Failed to draw rectangle");

    // Draw Rect
    let rect = rectangle_2d::Rectangle2D {
        pos_x: 300,
        pos_y: 400,
        width: 100,
        height: 200,
        col: Color::RGB(0, 255, 0),
    };
    rect.draw(canvas);

    // Draw Rect
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let rect = Rect::new(150, 150, 200, 150); // X, Y, Width, Height
    canvas.fill_rect(rect).expect("Failed to draw rectangle");
}
