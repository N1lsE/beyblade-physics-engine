use crate::winsdl::Winsdl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::{
    time::{Duration, Instant},
    usize,
};

mod rectangle_2d;
use rectangle_2d::{Drawable, Rectangle2D};

mod winsdl;

fn main() {
    const TARGET_FPS: u32 = 60;
    const FRAME_TIME: Duration = Duration::from_secs(1 / TARGET_FPS as u64);
    const SCREEN_WIDTH: usize = 800;
    const SCREEN_HEIGHT: usize = 600;

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

    let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();

    let rec1 = Box::new(Rectangle2D {
        pos_x: 300,
        pos_y: 400,
        width: 100,
        height: 200,
        col: Color::RGB(0, 255, 0),
    });
    drawables.push(rec1);

    let rec2 = Box::new(rectangle_2d::Rectangle2D {
        pos_x: 100,
        pos_y: 400,
        width: 100,
        height: 200,
        col: Color::RGB(0, 255, 0),
    });
    drawables.push(rec2);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        update(&mut canvas, &mut background, &drawables, FRAME_TIME);
    }
}

fn update(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    background: &mut sdl2::render::Texture<'_>,
    drawables: &Vec<Box<dyn Drawable>>,
    frame_time: Duration,
) {
    let frame_start = Instant::now();

    // Clear Canvas
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Draw on Canvas
    canvas
        .with_texture_canvas(background, |texture_canvas| {
            draw(texture_canvas, drawables);
        })
        .unwrap();

    // Copy backgroundbuffe on forgroundbuffer
    canvas.copy(background, None, None).unwrap();

    // Render Canvas
    canvas.present();

    // Limit FPS
    let elapsed = frame_start.elapsed();
    if elapsed < frame_time {
        std::thread::sleep(frame_time - elapsed);
    }
}

fn draw(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    drawables: &Vec<Box<dyn Drawable>>,
) {
    // Draw Background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Draw drawables
    for drawable in drawables {
        drawable.draw(canvas);
    }
}
