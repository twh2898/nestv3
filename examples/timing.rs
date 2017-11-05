extern crate glium;
use glium::glutin;

extern crate nestv3;
use nestv3::*;

use std::time::Instant;

fn main() {
	let mut event_loop = glutin::EventsLoop::new();
	let mut app = Window::new("Timing Example", 640, 480, &event_loop).unwrap();

	let mut running = true;
	let mut last = Instant::now();
	
	while running {
		let curr = Instant::now();
		let delta = nestv3::as_sec(curr - last);
		last = curr;

		let mut frame = app.next_frame();
		{
			frame.clear_color([0.0, 0.0, 0.0, 1.0]);
			/* Draw Code Here */
		}
		frame.finish().unwrap();

		event_loop.poll_events(|ev| {
			use glium::glutin::{Event, WindowEvent};

			match ev {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::Closed => running = false,
					_ => (),
				}
				_ => (),
			}
		});
	}
}
