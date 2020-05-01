#[macro_use]
extern crate glium;
mod triangle;

use glium::{Display, Surface};

fn draw(display: &Display) {
    // Gets the buffered surface.
    let mut target = display.draw();

    // Clears and colours the buffered surface.
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    // Draw the triangle.
    target.draw(
        &triangle::get_vertex_buffer(&display),
        &triangle::get_index_buffer(),
        &triangle::get_shader_program(&display),
        &glium::uniforms::EmptyUniforms,
        &Default::default()
    ).unwrap();

    // Swaps the buffered surface to the window.
    target.finish().unwrap();
}

// This is all our window code from the previous tutorial.
fn main() {
    use glium::glutin;

    // Creating a window.
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Keeping the window open and listening for events.
    event_loop.run(move |ev, _, control_flow| {

        // Do drawing here.
        draw(&display);

        // Handling our fps stuff.
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil((next_frame_time));

        // Handling the event.
        match ev {

            // Handle window specific events.
            glutin::event::Event::WindowEvent { event, .. } => match event {

                // If user tries to close the window, then close it.
                glutin::event::WindowEvent::CloseRequested=> {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },

                // Ignore all other events for now.
                _ => return
            },

            // Ignore all other events.
            _ => return
        }
    })
}
