extern crate glium;
use glium::glutin;

extern crate nestv3;
use nestv3::*;

fn main() {
	let mut event_loop = glutin::EventsLoop::new();
	let app = Window::new("Image Example", 640, 480, &event_loop).unwrap().with_dim_mode(DimensionMode::Scaled);
	let img = app.load_image("examples/res/city.jpg").unwrap();

	// All vertex coordinates are in pixels
	let points: Vec<Vertex> = vec![
		(0.0, 0.0, [0.0, 0.0]),
		(0.0, 1.0, [0.0, 1.0]),
		(1.0, 0.0, [1.0, 0.0]),
		(1.0, 1.0, [1.0, 1.0]),
	].iter().map(|p| Into::<Vertex>::into(*p)).collect();

	let mut running = true;
	while running {
		let mut frame = app.next_frame();
		{
			frame.clear_color([0.0, 0.0, 0.0, 1.0]);
			/* Draw Code Here */
			frame.draw_image(points.clone(), img.clone());
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
