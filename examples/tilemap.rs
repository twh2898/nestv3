extern crate glium;
use glium::glutin;

extern crate nestv3;
use nestv3::*;

fn main() {
	let mut event_loop = glutin::EventsLoop::new();
	let app = Window::new("TileMap Example", 640, 480, &event_loop).unwrap()
		.with_dim_mode(DimensionMode::Scaled);
	let img = app.load_image("examples/res/tilemap.jpg").unwrap();
	let atlas = TextureAtlas::new(8, 8);

	// All vertex coordinates are in pixels
	let points: Vec<Vec<Vertex>> = vec![
		0u16; 16
	].iter().enumerate().map(|(i, p)| {
		let uvw = atlas.uvw(*p);
		let x = (i % 4) as f64;
		let y = (i / 4) as f64;

		let x = x / 4.0;
		let y = y / 4.0;

		vec![
			(x, y, [uvw[0], uvw[1]]),
			(x + 0.25, y, [uvw[2], uvw[1]]),
			(x, y + 0.25, [uvw[0], uvw[3]]),
			(x + 0.25, y + 0.25, [uvw[2], uvw[3]]),
		].iter().map(|point| Into::<Vertex>::into(*point)).collect::<Vec<Vertex>>()
	}).collect();

	let mut running = true;
	while running {
		let mut frame = app.next_frame();
		{
			frame.clear_color([0.0, 0.0, 0.0, 1.0]);
			/* Draw Code Here */
			for rect in points.iter() {
				frame.draw_image(rect.clone(), img.clone());
			}
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
