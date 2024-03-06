use sdl2::{pixels::Color, rect::Rect, render::Canvas, sys::Button1, video::Window};

mod Shapes;
use Shapes::{
    shapes_2d::{Button, Rectangle},
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
        Box::new(|| {
            println!("button1 got clicked!");
        }),
    ));
    shapes.push(&*button1);
    clickable.push(&*button1);
    drawable.push(&*button1);

    // Window Loop
    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                sdl2::event::Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => {
                    println!("x:{} y:{}", x, y);
                    for item in clickable.iter() {
                        if item.is_clicked(x, y) {
                            item.on_click();
                        }
                    }
                }
                _ => {}
            }
        }

        render(&mut canvas, &mut drawable);
    }
}

fn render(canvas: &mut Canvas<Window>, drawable: &mut Vec<&dyn Drawable>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for item in drawable.iter() {
        item.render(canvas);
    }

    canvas.present();
}
