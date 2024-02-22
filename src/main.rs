use crate::winsdl::WinsdlGl;
use sdl2::event::Event;
use std::time::Instant;

mod winsdl;

fn main() {
    let mut winsdl = WinsdlGl::new(800, 600).unwrap();

    let start = Instant::now();

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        let time_elapsed = (start.elapsed().as_secs_f32().sin() + 1.0) / 2.0;
        unsafe {
            gl::ClearColor(time_elapsed, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        winsdl.window.gl_swap_window();
    }
}
