use crate::winsdl::Winsdl;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::{
    time::{Duration, Instant},
    usize,
};

mod rectangle_2d;
use rectangle_2d::{DOType, DrawableObject, Rectangle2D};

mod winsdl;

fn main() {
    const TARGET_FPS: u32 = 60;
    const FRAME_TIME: Duration = Duration::from_secs(1 / TARGET_FPS as u64);
    const SCREEN_WIDTH: usize = 1000;
    const SCREEN_HEIGHT: usize = 800;

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

    let mut drawables: Vec<Box<dyn DrawableObject>> = Vec::new();

    init(&mut drawables);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        update(&mut canvas, &mut background, &mut drawables, FRAME_TIME);
    }
}
fn init(drawables: &mut Vec<Box<dyn DrawableObject>>) {
    let rec1 = Box::new(Rectangle2D::new(
        1000,
        400,
        100,
        200,
        Color::RGB(0, 255, 0),
        DOType::Pipe,
    ));
    drawables.push(rec1);

    let rec2 = Box::new(Rectangle2D::new(
        2000,
        400,
        100,
        200,
        Color::RGB(0, 255, 0),
        DOType::Pipe,
    ));
    drawables.push(rec2);

    let rec3 = Box::new(Rectangle2D::new(
        3000,
        400,
        100,
        200,
        Color::RGB(0, 255, 0),
        DOType::Pipe,
    ));
    drawables.push(rec3);
}

fn update(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    background: &mut sdl2::render::Texture<'_>,
    drawables: &mut Vec<Box<dyn DrawableObject>>,
    frame_time: Duration,
) {
    let frame_start = Instant::now();

    // Clear Canvas
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Update Pipes
    for drawable in drawables.iter_mut() {
        match drawable.get_object_type() {
            DOType::Pipe => {
                if drawable.get_pos_x() <= 0 - drawable.get_width() as i32 {
                    drawable.set_pos_x(3000);
                } else {
                    drawable.set_pos_x(drawable.get_pos_x() - 1);
                }
            }
            DOType::Player => {}
            _ => {
                panic!("Not Implemented DOType update")
            }
        }
    }

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
    drawables: &Vec<Box<dyn DrawableObject>>,
) {
    // Draw Background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();

    // Draw drawables
    for drawable in drawables {
        drawable.draw(canvas);
    }
}
