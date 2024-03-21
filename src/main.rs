use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::time::{Duration, Instant};

mod my_shapes;
use my_shapes::{
    shapes_2d::Button,
    traits::{Clickable, Drawable, Shape},
};

fn main() {
    let sdl_content = sdl2::init().unwrap();
    let video_subsystem = sdl_content.video().unwrap();
    let window = video_subsystem
        .window("Beyblade Physics Engine", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut event_pump = sdl_content.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut shapes: Vec<&dyn Shape> = Vec::new();
    let mut clickable: Vec<&dyn Clickable> = Vec::new();
    let mut drawable: Vec<&dyn Drawable> = Vec::new();

    let button1 = Box::new(Button::new(
        "ClickMe".to_string(),
        Rect::new(200, 100, 100, 50),
        Color::RGB(255, 0, 0),
        Color::RGB(0, 0, 0),
        true,
        Box::new(|| println!("Button got clicked!")),
    ));
    shapes.push(&*button1);
    clickable.push(&*button1);
    drawable.push(&*button1);

    // Window Loop
    let mut angle = 0.0;

    const TARGET_FPS: u32 = 60;
    let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FPS as f32);
    let mut last_frame_time = Instant::now();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                sdl2::event::Event::MouseButtonUp {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x,
                    y,
                } => {
                    println!("x:{} y:{}", x, y);
                    for item in clickable.iter() {
                        if item.is_clicked(x, y) {
                            item.on_click().unwrap();
                        }
                    }
                }
                _ => {}
            }
        }

        render(&mut canvas, &mut drawable, angle);
        angle += 0.02;

        // FPS limiting - sleep if frame duration is less than target
        let elapsed = last_frame_time.elapsed();
        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }
        last_frame_time = Instant::now();
    }
}

fn render(canvas: &mut Canvas<Window>, drawable: &mut [&dyn Drawable], angle: f32) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for item in drawable.iter() {
        item.render(canvas).unwrap();
    }
    // let mut rectangle: Vec<Point> = Vec::new();
    // rectangle.push(Point::new(150, 150));
    // rectangle.push(Point::new(50, 150));
    // rectangle.push(Point::new(50, 50));
    // rectangle.push(Point::new(150, 50));

    // Create
    let mut rectangle: Vec<Point> = vec![
        Point::new(150, 150),
        Point::new(50, 150),
        Point::new(50, 50),
        Point::new(150, 50),
    ];
    // Rotate
    for point in rectangle.iter_mut() {
        *point = rotate_point(*point, Point::new(70, 120), angle * 4.5);
    }
    // Draw
    let rec_len = rectangle.len() - 1;
    for i in 0..(rec_len) {
        let p1 = rectangle[i];
        let p2 = rectangle[i + 1];
        canvas.draw_line(p1, p2).unwrap();
    }
    canvas.draw_line(rectangle[0], rectangle[rec_len]).unwrap();

    // Create
    let mut rectangle: Vec<Point> = vec![
        Point::new(450, 450),
        Point::new(400, 450),
        Point::new(400, 200),
        Point::new(450, 200),
    ];
    // Rotate
    for point in rectangle.iter_mut() {
        *point = rotate_point(*point, Point::new(310, 313), -angle * 0.5);
    }
    // Draw
    canvas.set_draw_color(Color::RGB(255, 0, 255));
    let rec_len = rectangle.len() - 1;
    for i in 0..(rec_len) {
        let p1 = rectangle[i];
        let p2 = rectangle[i + 1];
        canvas.draw_line(p1, p2).unwrap();
    }
    canvas.draw_line(rectangle[0], rectangle[rec_len]).unwrap();

    // Create
    let mut rectangle: Vec<Point> = vec![
        Point::new(350, 350),
        Point::new(400, 350),
        Point::new(400, 300),
        Point::new(350, 300),
    ];
    // Rotate
    for point in rectangle.iter_mut() {
        *point = rotate_point(*point, Point::new(210, 313), angle * 1.75);
    }
    // Draw
    canvas.set_draw_color(Color::RGB(0, 0, 255));
    let rec_len = rectangle.len() - 1;
    for i in 0..(rec_len) {
        let p1 = rectangle[i];
        let p2 = rectangle[i + 1];
        canvas.draw_line(p1, p2).unwrap();
    }
    canvas.draw_line(rectangle[0], rectangle[rec_len]).unwrap();

    canvas.present();
}

fn rotate_point(point: Point, origin: Point, angle: f32) -> Point {
    let old_x = (point.x() - origin.x()) as f32;
    let old_y = (point.y() - origin.y()) as f32;

    let x: i32 = ((old_x * angle.cos()) + (old_y * (-angle.sin()))).round() as i32 + origin.x();
    let y: i32 = ((old_x * angle.sin()) + (old_y * angle.cos())).round() as i32 + origin.y();

    Point::new(x, y)
}
